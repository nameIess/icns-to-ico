@echo off
setlocal

echo ========================================
echo   ICNS to ICO - Release Build
echo ========================================
echo.

:: Get version from git tag, fallback to "dev"
for /f "delims=" %%v in ('git describe --tags --always --dirty 2^>nul') do set "VERSION=%%v"
if not defined VERSION set "VERSION=dev"

:: Get short commit hash
for /f "delims=" %%c in ('git rev-parse --short HEAD 2^>nul') do set "COMMIT=%%c"
if not defined COMMIT set "COMMIT=none"

set "BINARY=icns-to-ico.exe"

:: Step 1: Check Rust toolchain
echo [1/3] Checking Rust toolchain...
where cargo >nul 2>nul
if errorlevel 1 (
    echo      ERROR: cargo not found. Install Rust from https://rustup.rs/
    exit /b 1
)
for /f "delims=" %%r in ('rustc --version 2^>nul') do set "RUSTVER=%%r"
echo      %RUSTVER%
echo      Done.

:: Step 2: Build with cargo
echo [2/3] Compiling %BINARY%...
echo      Version: %VERSION%
echo      Commit:  %COMMIT%
set "MINGW_PATH=C:\msys64\mingw64\bin"
if exist "%MINGW_PATH%\gcc.exe" (
    set "PATH=%MINGW_PATH%;%PATH%"
)
cargo build --release
if errorlevel 1 (
    echo      ERROR: Build failed. Run 'cargo build --release' for details.
    exit /b 1
)
echo      Done.

:: Step 3: Copy and report
echo [3/3] Build complete.
if exist "target\release\%BINARY%" (
    copy /y "target\release\%BINARY%" "%BINARY%" >nul
    for %%F in (%BINARY%) do set "SIZE=%%~zF"
)
setlocal enabledelayedexpansion
set /a "KB=!SIZE! / 1024"
set /a "MB=!KB! / 1024"
echo.
echo ========================================
echo   Output:  %BINARY%
echo   Size:    !KB! KB (~!MB! MB)
echo   Version: %VERSION%
echo   Commit:  %COMMIT%
echo ========================================
endlocal

endlocal
