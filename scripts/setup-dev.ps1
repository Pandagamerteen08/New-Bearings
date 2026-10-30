$ErrorActionPreference = "Stop"

Write-Host "Checking the Recall development environment..." -ForegroundColor Cyan

foreach ($Command in @("rustc", "cargo", "rustup", "git")) {
    if (Get-Command $Command -ErrorAction SilentlyContinue) {
        Write-Host "$Command is available." -ForegroundColor Green
    }
    else {
        Write-Host "$Command was not found on PATH." -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "If Rust is missing, visit https://www.rust-lang.org/tools/install"
Write-Host "Git is recommended for source control."