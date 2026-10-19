$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

if (-not (Test-Path ".venv")) {
    Write-Host "Creating Python virtual environment..." -ForegroundColor Yellow
    python -m venv .venv
}

. .\.venv\Scripts\Activate.ps1
pip install --upgrade pip

Write-Host "Installing pre-commit and setting up hooks..." -ForegroundColor Yellow
pip install pre-commit
pre-commit install

Write-Host "Dependencies installed and pre-commit hooks configured." -ForegroundColor Green
