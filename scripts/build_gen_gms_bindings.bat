@echo off
setlocal enabledelayedexpansion

REM =====================================================================
REM gen-gms-bindings - Build Script
REM
REM Builds the gen-gms-bindings Rust tool that generates GML schema
REM files from SpacetimeDB WASM modules or pre-extracted JSON.
REM
REM Usage:
REM   build_gen_gms_bindings.bat            Build release (default)
REM   build_gen_gms_bindings.bat --debug    Build debug
REM   build_gen_gms_bindings.bat --help     Show help
REM =====================================================================

set "SCRIPT_DIR=%~dp0"
set "SDK_ROOT=%SCRIPT_DIR%.."
set "GEN_DIR=%SDK_ROOT%\gen-gms-bindings"
set "BUILD_MODE=release"

REM Parse arguments
:parse_args
if "%1"=="" goto :done_args
if /i "%1"=="--debug" set "BUILD_MODE=debug"
if /i "%1"=="--help" goto :show_help
shift
goto :parse_args

:show_help
echo Usage: build_gen_gms_bindings.bat [OPTIONS]
echo.
echo Options:
echo   --debug     Build in debug mode (default: release)
echo   --help      Show this help
echo.
echo Examples:
echo   build_gen_gms_bindings.bat
echo   build_gen_gms_bindings.bat --debug
exit /b 0

:done_args

echo ============================================
echo gen-gms-bindings - Build
echo ============================================
echo Source: %GEN_DIR%
echo Mode:   %BUILD_MODE%
echo.

REM Build
echo --- Building gen-gms-bindings (%BUILD_MODE%) ---
pushd "%GEN_DIR%"
if "%BUILD_MODE%"=="release" (
    cargo build --release 2>&1
) else (
    cargo build 2>&1
)
if errorlevel 1 (
    echo ERROR: Build failed
    popd
    exit /b 1
)
popd

echo.
echo ============================================
echo Build successful!
echo.

REM Show output location
if "%BUILD_MODE%"=="release" (
    set "EXE_PATH=%GEN_DIR%\target\release\gen-gms-bindings.exe"
) else (
    set "EXE_PATH=%GEN_DIR%\target\debug\gen-gms-bindings.exe"
)

if exist "!EXE_PATH!" (
    echo Executable: !EXE_PATH!
    echo.

    REM Copy to GMS2 datafiles
    set "DATAFILES_DIR=%SDK_ROOT%\source\SpacetimeDB_gml\datafiles"
    if not exist "!DATAFILES_DIR!" mkdir "!DATAFILES_DIR!"
    copy /Y "!EXE_PATH!" "!DATAFILES_DIR!\gen-gms-bindings.exe" >nul
    if errorlevel 1 (
        echo WARNING: Failed to copy gen-gms-bindings.exe to !DATAFILES_DIR!
    ) else (
        echo Copied to: !DATAFILES_DIR!\gen-gms-bindings.exe
    )
    echo.

    echo Next step: Generate GML bindings, e.g.:
    echo   source\SpacetimeDB_gml\datafiles\generate_gml_bindings.bat --wasm path\to\module.wasm --out output.gml
) else (
    echo WARNING: Expected executable not found at !EXE_PATH!
    echo   It may be in a different target directory if cross-compiling.
)

echo ============================================
endlocal
