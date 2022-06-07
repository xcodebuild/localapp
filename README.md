![GitHub top language](https://img.shields.io/github/languages/top/xcodebuild/localapp?style=for-the-badge)
# localapp
Rust CLI to convert webpage into desktop app with tauri under 10 MB.

## Install

- Install rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
  
- Install `localapp` with `cargo`
```shell
cargo install localapp
```

## Usage

```
localapp <you-website-url>
# For example localapp https://flomoapp.com
```

## Why localapp

- No electron, `2.6 MB dmg` for flomo example.
- Acceptable memory footprint with system webview.
- Cross-platform based on rust toolchain and tauri.

