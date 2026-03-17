# Proto WASM Plugins

[![CI](https://github.com/muuvmuuv/proto-plugins/actions/workflows/release.yml/badge.svg)](https://github.com/muuvmuuv/proto-plugins/actions/workflows/release.yml) [![License: MIT](https://img.shields.io/github/license/muuvmuuv/proto-plugins)](LICENSE) [![GitHub downloads](https://img.shields.io/github/downloads/muuvmuuv/proto-plugins/total)](https://github.com/muuvmuuv/proto-plugins/releases)

A collection of [proto](https://moonrepo.dev/proto) WASM plugins for managing CLI
tools. Requires proto v2 / moon v2.

## Plugins

| Tool | Description | Plugin | Latest |
| --- | --- | --- | --- |
| [Gitleaks](https://github.com/gitleaks/gitleaks) | Secret scanner for git repos | `gitleaks_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=gitleaks_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=gitleaks_tool) |
| [jq](https://github.com/jqlang/jq) | JSON processor | `jq_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=jq_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=jq_tool) |
| [just](https://github.com/casey/just) | Command runner | `just_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=just_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=just_tool) |
| [yq](https://github.com/mikefarah/yq) | YAML/JSON/XML processor | `yq_tool` | [![release](https://img.shields.io/github/v/release/muuvmuuv/proto-plugins?filter=yq_tool-*&label=)](https://github.com/muuvmuuv/proto-plugins/releases?q=yq_tool) |

## Usage

Add the plugin to your `.prototools`:

```toml
[plugins]
gitleaks = "github://muuvmuuv/proto-plugins/gitleaks_tool"
jq = "github://muuvmuuv/proto-plugins/jq_tool"
just = "github://muuvmuuv/proto-plugins/just_tool"
yq = "github://muuvmuuv/proto-plugins/yq_tool"
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
