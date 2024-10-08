#!/bin/bash

# Activate virtual environment
source venv/bin/activate

pip install -r requirements.txt

# Run PyInstaller with your spec file
pyinstaller --onefile maestro/main.py --name Maestro

echo "Build complete. Check the 'dist' directory for the executable."

echo "Running the executable..."

./dist/Maestro

# Deactivate virtual environment
deactivate
