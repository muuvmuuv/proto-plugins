# Claude notes for this repo

proto WASM plugins in Rust, one crate per tool under `tools/*`.
Build target is `wasm32-wasip1`. Each plugin implements the functions in the
[proto PDK](https://moonrepo.dev/docs/proto/wasm-plugin) — `register_tool`,
`load_versions`, `download_prebuilt`, `locate_executables`.

## Workflow

- Use `just` for everything local: `just build`, `just test`, `just test-tool <name>`.
- Don't run `cargo test` directly; the justfile wires up `PROTO_LOOKUP_DIR` so
  the test harness can find `proto-shim` when proto was installed via Homebrew.
- Commits follow conventional commits (`fix:`, `feat:`, `docs:`).
- Releases: bump the crate's `Cargo.toml` version, commit, tag `<name>_tool-v<ver>`,
  push the tag. The `release.yml` workflow builds the WASM and attaches it to
  a GitHub release.
- Remember to bump the direct-download URL in `README.md` when releasing.

## proto plugin gotchas (non-obvious)

These are load-bearing; re-learning them costs an afternoon.

### The PDK version is a hard compatibility contract with the proto CLI

Plugin function input crosses the WASM boundary as JSON, so proto adding or
removing a required context field breaks every plugin built against the other
side of that change. The symptom is a bare serde error naming the field, with
no hint that versions are involved:

- proto 0.60 removed `PluginUnresolvedContext.tool_dir` → plugins on
  `proto_pdk_api` ≤ 0.31 fail `load_versions` with ``missing field `tool_dir` ``.
- proto ≤ 0.55 predates `working_dir` → plugins on the newer PDK fail with
  ``missing field `working_dir` ``.

This breaks *installed* users, not just this repo: proto is auto-upgraded by
`moonrepo/setup-toolchain` and `proto upgrade`, so a proto release can break
every downstream CI overnight while local machines keep working off the
`~/.proto/plugins` cache. When it happens, bump `proto_pdk`, `proto_pdk_api`,
and `proto_pdk_test_utils` together and re-release **all** plugins — they share
the workspace PDK and break as a set.

Keep `minimum_proto_version` in `register_tool` honest. Establish it by
installing a plugin against downloaded proto CLI tarballs (bisect the
releases); note that proto 0.56 can't be used for this, as it blows up on
long `file://` locator paths for unrelated reasons.

### PATH activation depends on filename matching the tool id

`proto activate` only adds `~/.proto/tools/<tool>/<ver>/` to `PATH` if that dir
contains a file whose name equals the tool id. If the upstream release
extracts a platform-suffixed binary (e.g. `yq_darwin_arm64`), activate silently
skips the dir and invocations fall through to `~/.proto/shims/<tool>`.

The `proto-shim` binary in proto ≤ 0.56.x then routes commands through
`bash -c <flattened-string>`, which mangles shell-special chars (`( )`, `|`,
`|=`, quotes) in arguments. The user's first-choice symptom is usually a
`bash: -c: line 0: syntax error` on an otherwise valid command.

Fix it at the plugin layer. Options in order of preference:

1. Download a single-file binary and name it after the tool (proto's non-archive
   install path copies it to `<install_dir>/<tool>` automatically — see
   `yq_tool`'s `download_prebuilt`).
2. Implement the `unpack_archive` hook and rename the file after extraction.

### `post_install` hook is defined in the PDK but NOT called by `proto_core`

As of `proto_core` 0.55/0.56 there are zero references to `PostInstall` /
`post_install` plugin function invocation in the core crate. The PDK exposes
`HookFunction::PostInstall = "post_install"` and the test wrapper has a manual
invoker, but the real install flow never calls it. Don't use it.

Invocation points that DO exist for plugins during install:
`register_tool`, `load_versions`, `resolve_version`, `download_prebuilt`,
`verify_checksum`, `unpack_archive`, `native_install`, `locate_executables`.

### The per-version lockfile pins URL + checksum

`~/.proto/tools/<tool>/manifest.json` stores a `lock` object per installed
version with the download URL and checksum. Changing `download_prebuilt`
breaks upgrades: the next install downloads the new URL but compares it
against the old expected hash.

Tell users to `proto uninstall <tool> <ver>` + reinstall when releasing a
plugin change that affects the download URL. Mention it in the release notes
and the commit body.

### Filesystem access from WASM

Don't try pure-Rust tar/zip extraction, and don't try `std::fs` writes from
inside a hook. The WASI sandbox restricts filesystem writes, and
`std::fs::update_permissions` isn't implemented. If you truly need to
manipulate files, the only path is `exec_command` host function — but prefer
architectural fixes (e.g. download a differently-named asset) over shelling out.

## Checksum quirks per tool

- `jq`, `just`, `gitleaks`, `lefthook`, `maestro`: standard `sha256sum.txt`,
  proto handles natively via `checksum_url`.
- `yq`: non-standard multi-hash file (30+ columns per row). The plugin fetches
  and parses it to extract column index 18 (SHA-256). Raw binary lookup matches
  row `yq_<os>_<arch>`; don't accidentally match the `.tar.gz` row.
- `sfw`: no upstream checksum file at all. The plugin omits the `checksum`
  field; integrity comes from HTTPS to github.com. Per-asset SHA-256 digests
  exist on the GitHub Releases REST API (`digest: sha256:...` field) if we
  ever want to add verification later.
