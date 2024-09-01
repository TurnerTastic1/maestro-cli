#!/bin/bash

# Activate virtual environment
source venv/bin/activate

pip install -r requirements.txt

# Run PyInstaller with your spec file
pyinstaller --onefile servicegenie/main.py --name ServiceGenie

echo "Build complete. Check the 'dist' directory for the executable."

echo "Running the executable..."

./dist/ServiceGenie

# Deactivate virtual environment
deactivate
