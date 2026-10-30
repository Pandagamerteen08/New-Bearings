$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

cargo run
if ($LASTEXITCODE -ne 0) {
    throw "Recall failed to run."
}