# Proto WASM Plugins

A collection of [proto](https://moonrepo.dev/proto) WASM plugins for managing CLI
tools. Requires proto v2 / moon v2.

## Plugins

| Tool | Description | Plugin |
| --- | --- | --- |
| [Gitleaks](https://github.com/gitleaks/gitleaks) | Secret scanner for git repos | `gitleaks_tool` |
| [just](https://github.com/casey/just) | Command runner | `just_tool` |

## Usage

Add the plugin to your `.prototools`:

```toml
[plugins]
gitleaks = "github://muuvmuuv/proto-plugins/gitleaks_tool"
just = "github://muuvmuuv/proto-plugins/just_tool"
```

Then install:

```sh
proto install gitleaks
proto install just
```

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (toolchain is pinned in `rust-toolchain.toml`)
- [proto](https://moonrepo.dev/proto)

### Build

```sh
cargo build --target wasm32-wasip1
```

### Test locally

Uncomment the plugins in `.prototools`, build, then run:

```sh
proto --log trace install gitleaks
proto --log trace install just
```

### Release

Tag the crate you want to release and push:

```sh
# Single plugin repo style
git tag gitleaks_tool-v0.1.0
git push --tags
```

The GitHub Actions workflow will build the WASM binary, optimize it, and create a
release with the `.wasm` asset attached.

## License

MIT
