$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

Write-Host "Checking formatting..." -ForegroundColor Cyan
cargo fmt --check
if ($LASTEXITCODE -ne 0) {
    throw "Formatting check failed. Run cargo fmt and try again."
}

Write-Host "Checking compilation..." -ForegroundColor Cyan
cargo check --all-targets
if ($LASTEXITCODE -ne 0) {
    throw "Cargo check failed."
}

Write-Host "Running tests..." -ForegroundColor Cyan
cargo test
if ($LASTEXITCODE -ne 0) {
    throw "Tests failed."
}

Write-Host "Running Clippy..." -ForegroundColor Cyan
cargo clippy --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) {
    throw "Clippy reported problems."
}

Write-Host "Type-checking and building the frontend..." -ForegroundColor Cyan
npm --prefix frontend run build
if ($LASTEXITCODE -ne 0) {
    throw "The frontend build failed."
}

Write-Host "All checks passed." -ForegroundColor Green
