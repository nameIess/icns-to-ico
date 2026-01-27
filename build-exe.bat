@echo off
REM Build the executable using convert.py (uses icon.ico from this directory if present)
cd /d "%~dp0"
python "%~dp0convert.py" --build
pause
