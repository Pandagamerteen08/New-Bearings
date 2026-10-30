$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

Write-Host "Building Recall..." -ForegroundColor Cyan

cargo build
if ($LASTEXITCODE -ne 0) {
    throw "Recall debug build failed."
}

Write-Host "Build completed successfully." -ForegroundColor Green