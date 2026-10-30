$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

Write-Host "Checking formatting..." -ForegroundColor Cyan
cargo fmt --check
if ($LASTEXITCODE -ne 0) {
    throw "Formatting check failed. Run cargo fmt and try again."
}

Write-Host "Checking compilation..." -ForegroundColor Cyan
cargo check
if ($LASTEXITCODE -ne 0) {
    throw "Cargo check failed."
}

Write-Host "Running tests..." -ForegroundColor Cyan
cargo test
if ($LASTEXITCODE -ne 0) {
    throw "Tests failed."
}

Write-Host "All available checks passed." -ForegroundColor Green