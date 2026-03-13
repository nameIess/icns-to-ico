# ICNS to ICO Converter

A fast, native Windows TUI tool to batch convert Apple `.icns` icon files to Windows `.ico` format — written in Rust.

## Features

- 🖥️ **Interactive TUI** — beautiful terminal interface powered by [Ratatui](https://ratatui.rs/)
- ⚡ **Parallel Processing** — converts multiple files concurrently with Rayon
- 🎨 **Multi-Resolution ICO** — outputs proper ICO files with 16/32/48/64/128/256px sizes
- 📁 **Auto Folder Management** — creates input/output directories in your Downloads folder automatically
- 📦 **Tiny Executable** — native Rust binary, no runtime dependencies (~2–5 MB)
- 🛡️ **No Antivirus Warnings** — natively compiled, no PyInstaller packaging

## Quick Start

### Option 1: Download

Download the latest `icns-to-ico.exe` from [GitHub Releases](https://github.com/nameIess/icns-to-ico/releases).

Verify integrity:
```powershell
# Compare with checksums.txt from the release
(Get-FileHash -Algorithm SHA256 icns-to-ico.exe).Hash
```

### Option 2: Build from Source

Prerequisites: [Rust 1.60+](https://rustup.rs/)

```bash
git clone https://github.com/nameIess/icns-to-ico.git
cd icns-to-ico
cargo build --release
# Executable: target\release\icns-to-ico.exe
```

## How It Works

1. Run `icns-to-ico.exe` — the TUI launches
2. The `Downloads/icons/icns` input folder opens automatically
3. **Place your `.icns` files** there
4. Press **Enter** in the TUI to start conversion
5. Watch the live conversion log — `[OK]` for success, `[ERR]` for failures
6. The `Downloads/icons/ico` output folder opens automatically when done
7. Press **q** to quit

## Getting ICNS Icons

Download high-quality macOS icons from:
- [macOS Icons](https://macosicons.com/#/)

## 🛠 Tech Stack

| | |
|---|---|
| Language | Rust 2021 |
| TUI | [Ratatui](https://ratatui.rs/) + [Crossterm](https://github.com/crossterm-rs/crossterm) |
| Image processing | [image](https://github.com/image-rs/image) + [icns](https://docs.rs/icns) |
| Parallelism | [Rayon](https://github.com/rayon-rs/rayon) |
| Resources | [embed-resource](https://github.com/nabijaczleweli/embed-resource) |
| Releases | GitHub Actions |

## 📁 Project Structure

```
icns-to-ico/
├── src/
│   ├── main.rs          # Entry point, terminal setup, event loop
│   ├── app.rs           # Application state & screen logic
│   ├── ui.rs            # Ratatui rendering
│   ├── converter.rs     # ICNS→ICO conversion logic (parallel)
│   └── filesystem.rs    # Directory management
├── resources/
│   ├── icon.ico         # Application icon
│   ├── app.manifest     # Windows manifest (DPI + compatibility)
│   └── app.rc           # Windows resource script (links icon + manifest)
├── .github/
│   └── workflows/
│       └── release.yml  # CI/CD release pipeline
├── build.rs             # Compiles app.rc → embeds icon into exe
├── Cargo.toml
└── README.md
```

## 🔐 Code Signing

To sign the release executable, add these repository secrets:
- `SIGNING_CERT_BASE64` — Base64-encoded `.pfx` certificate
- `SIGNING_CERT_PASSWORD` — Certificate password

Then uncomment the signing steps in `.github/workflows/release.yml`.

## Releasing

Tag a commit with a version tag to trigger an automated release:

```bash
git tag v1.1.0
git push origin v1.1.0
```

GitHub Actions will build the exe, compute a SHA-256 checksum, and publish a GitHub Release automatically.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

[MIT License](LICENSE)
