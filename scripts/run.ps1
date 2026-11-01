$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

$Tauri = Join-Path $PSScriptRoot "..\frontend\node_modules\.bin\tauri.cmd"

if (-not (Test-Path $Tauri)) {
    throw "The Tauri CLI is not installed. Run: npm --prefix frontend install"
}

# The Tauri CLI starts Vite, compiles the Rust binary and opens the window.
# Running `cargo run` directly would leave the interface without a dev server.
& $Tauri dev
if ($LASTEXITCODE -ne 0) {
    throw "New Bearings failed to start."
}
