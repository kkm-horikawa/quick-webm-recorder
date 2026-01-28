# Quick WebM Recorder

A lightweight cross-platform screen recorder built with Tauri v2 + ffmpeg.
Outputs high-quality, high-compression WebM (VP8) files with a macOS-style floating toolbar UI.

## Features

- **Full-screen recording** — one-click capture
- **WebM output** — VP8 codec, CRF 8, 1Mbps (small file, great quality)
- **GIF conversion** — palette-optimized GIF export
- **Floating toolbar** — minimal, always-on-top UI
- **Cross-platform** — Windows, macOS, Linux

## Requirements

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/)
- [ffmpeg](https://ffmpeg.org/) — either bundled in `src-tauri/ffmpeg/` or available in PATH

### Platform-specific

- **Linux**: `libwebkit2gtk-4.1-dev`, `librsvg2-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`
- **macOS**: Xcode Command Line Tools
- **Windows**: WebView2 (pre-installed on Windows 10/11)

## Development

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## ffmpeg Bundling

Place ffmpeg binaries in `src-tauri/ffmpeg/`:
- `ffmpeg` (macOS/Linux)
- `ffmpeg.exe` (Windows)

If not bundled, the app falls back to system ffmpeg.

## License

MIT. ffmpeg is used under LGPL.
