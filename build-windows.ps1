# Quick WebM Recorder - Windows Build Script
# Run this from PowerShell on Windows side.
#
# Prerequisites:
#   1. Rust: https://rustup.rs/
#   2. Node.js: https://nodejs.org/
#   3. WebView2 (pre-installed on Windows 10/11)
#
# Usage:
#   cd \\wsl.localhost\Ubuntu-22.04\home\m-horikawa\private\quick-webm-recorder
#   .\build-windows.ps1

Write-Host "=== Quick WebM Recorder - Windows Build ===" -ForegroundColor Cyan

# Check prerequisites
Write-Host "`nChecking prerequisites..." -ForegroundColor Yellow

$rustc = Get-Command rustc -ErrorAction SilentlyContinue
if (-not $rustc) {
    Write-Host "ERROR: Rust is not installed. Install from https://rustup.rs/" -ForegroundColor Red
    exit 1
}
Write-Host "  Rust: $(rustc --version)" -ForegroundColor Green

$node = Get-Command node -ErrorAction SilentlyContinue
if (-not $node) {
    Write-Host "ERROR: Node.js is not installed. Install from https://nodejs.org/" -ForegroundColor Red
    exit 1
}
Write-Host "  Node: $(node --version)" -ForegroundColor Green

# Install npm dependencies
Write-Host "`nInstalling npm dependencies..." -ForegroundColor Yellow
npm install
if ($LASTEXITCODE -ne 0) { exit 1 }

# Build
Write-Host "`nBuilding Tauri app..." -ForegroundColor Yellow
npm run tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Build complete! ===" -ForegroundColor Green
Write-Host "Installer: src-tauri\target\release\bundle\" -ForegroundColor Cyan
