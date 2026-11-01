$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

Write-Host "Checking the New Bearings development environment..." -ForegroundColor Cyan

$missing = @()

foreach ($Command in @("rustc", "cargo", "rustup", "git", "node", "npm")) {
    if (Get-Command $Command -ErrorAction SilentlyContinue) {
        Write-Host "$Command is available." -ForegroundColor Green
    }
    else {
        Write-Host "$Command was not found on PATH." -ForegroundColor Yellow
        $missing += $Command
    }
}

if (Test-Path "frontend\node_modules") {
    Write-Host "Frontend dependencies are installed." -ForegroundColor Green
}
else {
    Write-Host "Frontend dependencies are missing. Run: npm --prefix frontend install" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Rust:  https://www.rust-lang.org/tools/install"
Write-Host "Node:  https://nodejs.org/"
Write-Host "Tauri: https://v2.tauri.app/start/prerequisites/"
Write-Host ""

if ($missing.Count -gt 0) {
    Write-Host "Missing tools: $($missing -join ', ')" -ForegroundColor Red
    exit 1
}

Write-Host "The toolchain looks complete." -ForegroundColor Green
