@echo off

IF NOT EXIST .venv (
    echo "Creating Python virtual environment..."
    python -m venv .venv
)

CALL .venv\Scripts\activate.bat
pip install --upgrade pip

echo "Installing pre-commit and setting up hooks..."
pip install pre-commit
pre-commit install

echo "Dependencies installed and pre-commit hooks configured."
