# Proto WASM Plugins

[![Linux](https://github.com/muuvmuuv/proto-plugins/actions/workflows/ci-linux.yml/badge.svg)](https://github.com/muuvmuuv/proto-plugins/actions/workflows/ci-linux.yml) [![macOS](https://github.com/muuvmuuv/proto-plugins/actions/workflows/ci-macos.yml/badge.svg)](https://github.com/muuvmuuv/proto-plugins/actions/workflows/ci-macos.yml) [![Windows](https://github.com/muuvmuuv/proto-plugins/actions/workflows/ci-windows.yml/badge.svg)](https://github.com/muuvmuuv/proto-plugins/actions/workflows/ci-windows.yml) [![License: MIT](https://img.shields.io/github/license/muuvmuuv/proto-plugins)](LICENSE) [![GitHub downloads](https://img.shields.io/github/downloads/muuvmuuv/proto-plugins/total)](https://github.com/muuvmuuv/proto-plugins/releases)

A collection of [proto](https://moonrepo.dev/proto) WASM plugins for managing CLI
tools. Requires proto >= 0.57 / moon v2.

## Plugins

| Tool | Description | Plugin | Latest |
| --- | --- | --- | --- |
| [Gitleaks](https://github.com/gitleaks/gitleaks) | Secret scanner for git repos | `gitleaks_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=gitleaks_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=gitleaks_tool) |
| [jq](https://github.com/jqlang/jq) | JSON processor | `jq_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=jq_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=jq_tool) |
| [just](https://github.com/casey/just) | Command runner | `just_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=just_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=just_tool) |
| [lefthook](https://github.com/evilmartians/lefthook) | Git hook manager | `lefthook_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=lefthook_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=lefthook_tool) |
| [Maestro](https://maestro.dev) | Mobile & web UI testing (requires Java) | `maestro_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=maestro_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=maestro_tool) |
| [sfw](https://github.com/SocketDev/sfw-free) | Socket Firewall Free, a network security proxy for package managers | `sfw_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=sfw_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=sfw_tool) |
| [yq](https://github.com/mikefarah/yq) | YAML/JSON/XML processor | `yq_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=yq_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=yq_tool) |

## Usage

Add the plugin to your `.prototools`:

```toml
[plugins]
gitleaks = "github://muuvmuuv/proto-plugins/gitleaks_tool"
jq = "github://muuvmuuv/proto-plugins/jq_tool"
just = "github://muuvmuuv/proto-plugins/just_tool"
lefthook = "github://muuvmuuv/proto-plugins/lefthook_tool"
maestro = "github://muuvmuuv/proto-plugins/maestro_tool"
sfw = "github://muuvmuuv/proto-plugins/sfw_tool"
yq = "github://muuvmuuv/proto-plugins/yq_tool"
```

Then install:

```sh
proto install gitleaks
proto install just
proto install lefthook
```

### Direct download links

The `github://` locator requires proto to query the GitHub API to resolve plugin
versions. In CI environments without authentication, this can fail due to
[rate limiting](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)
(60 requests/hour for unauthenticated requests).

To avoid this, you can reference the WASM files directly by URL:

```toml
[plugins]
gitleaks = "https://github.com/muuvmuuv/proto-plugins/releases/download/gitleaks_tool-v0.3.0/gitleaks_tool.wasm"
jq = "https://github.com/muuvmuuv/proto-plugins/releases/download/jq_tool-v0.3.0/jq_tool.wasm"
just = "https://github.com/muuvmuuv/proto-plugins/releases/download/just_tool-v0.3.0/just_tool.wasm"
lefthook = "https://github.com/muuvmuuv/proto-plugins/releases/download/lefthook_tool-v0.2.0/lefthook_tool.wasm"
maestro = "https://github.com/muuvmuuv/proto-plugins/releases/download/maestro_tool-v0.2.0/maestro_tool.wasm"
sfw = "https://github.com/muuvmuuv/proto-plugins/releases/download/sfw_tool-v0.2.0/sfw_tool.wasm"
yq = "https://github.com/muuvmuuv/proto-plugins/releases/download/yq_tool-v0.4.0/yq_tool.wasm"
```

This downloads the WASM file directly without any GitHub API calls for plugin
resolution. Note that you will need to update the URLs manually when upgrading
plugin versions.

> [!TIP]
> If you prefer the `github://` locator, set the `GITHUB_TOKEN` environment
> variable in CI to get 5,000 requests/hour instead of 60. This also benefits
> the plugins themselves, which call the GitHub API to resolve tool versions.

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (toolchain is pinned in `rust-toolchain.toml`)
- [proto](https://moonrepo.dev/proto)

### Setup

Clone the repo and run:

```sh
just setup
```

This will:

1. Install the Rust toolchain and `wasm32-wasip1` target (via `rust-toolchain.toml`)
2. Build all WASM plugins (needed for `.prototools` to resolve local `file://` plugins)
3. Install pinned tool versions via `proto use`
4. Install [lefthook](https://github.com/evilmartians/lefthook) git hooks

### Commands

| Command | Description |
| --- | --- |
| `just setup` | Initial project setup |
| `just build` | Build all WASM plugins |
| `just test` | Build and run all tests |
| `just test-tool <name>` | Test a single plugin (e.g. `just test-tool jq`) |
| `just check` | Type-check without building |
| `just fmt` | Format code |
| `just lint` | Run clippy lints |

### Git hooks

[Lefthook](https://github.com/evilmartians/lefthook) is configured to run
automatically:

- **pre-commit**: `just lint` (clippy)
- **pre-push**: `just check` (fast compile check)

Tests are not hooked. They download ~900MB of real release artifacts per run —
`maestro.zip` is 300MB and two tests fetch it independently — so they run in CI
on every push and pull request instead.

### Testing

Every plugin has integration tests that verify version resolution, archive
download, checksum verification, and tool installation. Run them with:

```sh
just test
```

Tests compile the plugin to WASM first, then execute it inside a sandboxed proto
environment that downloads real release artifacts. All tests run in CI on every
push and pull request.

### Checksum verification

Most plugins verify download integrity using SHA-256 checksums. Some tools
require special handling:

- **jq, just, gitleaks, lefthook, maestro** -- standard `sha256sum.txt` format,
  handled natively by proto
- **yq** -- publishes a non-standard multi-hash checksums file (30+ algorithms
  per line); the plugin fetches and parses it during download to extract the
  SHA-256 hash
- **sfw** -- upstream publishes no checksum file, so the plugin relies on
  HTTPS to github.com for integrity

### Adding a new plugin

1. Create a new crate: `mkdir -p tools/<name>/src`
2. Add `Cargo.toml` (use an existing plugin as a template)
3. Implement the required plugin functions in `src/lib.rs`:
   - `register_tool` -- plugin metadata
   - `load_versions` -- fetch available versions (usually via `load_git_tags`)
   - `download_prebuilt` -- download URL, archive name, and checksum URL
   - `locate_executables` -- path to the executable within the install directory
4. Add tests using the `generate_download_install_tests!` and
   `generate_resolve_versions_tests!` macros
5. Add an entry to `.prototools` for local development
6. Run `just test-tool <name>` to verify

### Release

Tag the crate you want to release and push:

```sh
git tag <name>_tool-v<version>
git push origin main <name>_tool-v<version>
```

The GitHub Actions workflow will build the WASM binary, optimize it, and create a
release with the `.wasm` asset attached.

> [!IMPORTANT]
> Push **at most three tags per `git push`**. GitHub creates no push events at
> all when more than three tags arrive at once, so `git push --tags` after a
> workspace-wide bump lands every tag on the remote and builds nothing. The
> failure is silent: no workflow run, no release, no error. Recovering means
> deleting the remote tags and re-pushing in batches, because re-pushing a tag
> the remote already has is a no-op that fires no event either.

### FAQ

#### Tests fail with `MissingToolExecutable`

The file that `locate_executables` declares as the executable must actually exist
in the install dir after install. If the upstream release asset is a
platform-suffixed binary (e.g. `yq_darwin_arm64`), you have two options:

1. **Preferred:** if the project also publishes a raw single-file binary,
   download that instead of the archive and name it after the tool. proto's
   non-archive install path automatically copies it to `<install_dir>/<tool>`.
   (This is what `yq_tool` does.)
2. Implement the `unpack_archive` hook to extract the archive and rename the
   binary yourself.

Do **not** rely on the PDK's `post_install` hook for this — as of
`proto_core` 0.55/0.56 it is declared but never invoked by the core.

#### `which <tool>` resolves to `~/.proto/shims/<tool>` and complex args break

`proto activate` adds a tool's install dir to `PATH` only when the dir contains
a file whose name matches the tool id. If the extracted binary has a
platform-suffix (e.g. `yq_darwin_arm64`), activate skips the install dir and
invocations fall through to `~/.proto/shims/<tool>`. The `proto-shim` binary
in proto ≤ 0.56.x routes commands via `bash -c <flattened-string>`, which
mangles shell-special characters like `( )`, `|`, `|=`, `'` in arguments:

```
bash: -c: line 0: syntax error near unexpected token `('
```

Fix it on the plugin side by making the install dir contain a tool-named file
(see the `MissingToolExecutable` answer above). Do not try to work around it in
user code.

#### After changing `download_prebuilt`, installs fail with `mismatched_checksum`

proto persists the download URL and checksum for each installed version in
`~/.proto/tools/<tool>/manifest.json` (the "lockfile"). Upgrading a plugin that
changes the URL or checksum format does **not** invalidate this file, so the
next install compares the fresh download against a stale expected hash. Users
need to `proto uninstall <tool> <ver>` followed by `proto install <tool> <ver>`
once per installed version.

#### Installs fail with `missing field ...` after a proto upgrade

proto sends plugin function input as JSON that the PDK deserializes into structs,
so removing or adding a required context field is a breaking change on both
sides. A plugin built against a PDK older than the running proto fails with
`missing field <name>` (proto 0.60 dropped `tool_dir`, which older PDKs require);
a plugin built against a newer PDK than the running proto fails the same way in
reverse (proto 0.55 predates `working_dir`). Either way the message names the
field, not the version mismatch.

The fix is always to bump `proto_pdk`, `proto_pdk_api`, and
`proto_pdk_test_utils` in the workspace `Cargo.toml`, rebuild, and re-release
every plugin — they share the PDK, so they break together. Keep
`minimum_proto_version` in `register_tool` honest with the oldest proto the
current PDK still talks to.

#### Tests fail with `assertion failed: shim.path.exists()`

Proto needs to find the `proto-shim` binary to create shims. If you installed
proto via Homebrew, the shim binary is in `/opt/homebrew/bin/` instead of
`~/.proto/bin/`. The justfile handles this automatically via `PROTO_LOOKUP_DIR`.
Always run tests through `just test` instead of `cargo test` directly.

#### Tests fail with `UnknownAlgorithm` for checksums

Proto's built-in checksum parser only supports standard `sha256sum` / `sha512sum`
formats (`<hash>  <filename>`) and detects the algorithm from the file extension
or hash length. If a tool publishes checksums in a non-standard format, you need
to fetch and parse them yourself in `download_prebuilt` and provide the hash via
the `checksum` field instead of `checksum_url`.

## License

MIT
