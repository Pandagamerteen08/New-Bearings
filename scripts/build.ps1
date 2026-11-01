$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

$Tauri = Join-Path $PSScriptRoot "..\frontend\node_modules\.bin\tauri.cmd"

if (-not (Test-Path $Tauri)) {
    throw "The Tauri CLI is not installed. Run: npm --prefix frontend install"
}

Write-Host "Building an optimised New Bearings executable..." -ForegroundColor Cyan

# `tauri build` runs the frontend build, compiles with the custom-protocol
# feature so the interface is embedded, and skips bundling because
# bundle.active is false in tauri.conf.json.
& $Tauri build --no-bundle
if ($LASTEXITCODE -ne 0) {
    throw "The build failed."
}

Write-Host "Build completed: target\release\new-bearings.exe" -ForegroundColor Green
Write-Host "This executable runs without a development server." -ForegroundColor Green
