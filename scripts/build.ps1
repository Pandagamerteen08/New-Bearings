$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

Write-Host "Building the New Bearings frontend..." -ForegroundColor Cyan

npm --prefix frontend run build
if ($LASTEXITCODE -ne 0) {
    throw "The frontend build failed."
}

Write-Host "Compiling the New Bearings executable..." -ForegroundColor Cyan

# tauri.conf.json points at frontend/dist, so the frontend is built first.
cargo build
if ($LASTEXITCODE -ne 0) {
    throw "The Rust build failed."
}

Write-Host "Build completed: target\debug\new-bearings.exe" -ForegroundColor Green
Write-Host "No installer is produced: bundling is disabled in tauri.conf.json." -ForegroundColor Yellow
