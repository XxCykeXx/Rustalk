@echo off
setlocal enabledelayedexpansion

:: Rustalk Installation Script for Windows
:: Supports both Rust and JavaScript installations

echo 🦀 Welcome to Rustalk Installation!
echo    Secure P2P Terminal Chat
echo.

:: Function to check if command exists
where /q git
if errorlevel 1 (
    echo [ERROR] Git is required but not installed.
    echo Please install Git from: https://git-scm.com/
    pause
    exit /b 1
)

:: Parse command line arguments
set "INSTALL_TYPE=both"
if "%1"=="--rust" set "INSTALL_TYPE=rust"
if "%1"=="-r" set "INSTALL_TYPE=rust"
if "%1"=="--js" set "INSTALL_TYPE=js"
if "%1"=="-j" set "INSTALL_TYPE=js"
if "%1"=="--both" set "INSTALL_TYPE=both"
if "%1"=="-b" set "INSTALL_TYPE=both"
if "%1"=="--uninstall" goto :uninstall
if "%1"=="--help" goto :help
if "%1"=="-h" goto :help

goto :install_%INSTALL_TYPE%

:install_rust
echo [INFO] Installing Rust version...

:: Check if Rust is installed
where /q cargo
if errorlevel 1 (
    echo [WARNING] Rust not found. Please install Rust manually:
    echo https://rustup.rs/
    echo.
    echo After installing Rust, run this script again.
    pause
    exit /b 1
) else (
    echo [SUCCESS] Rust is already installed
)

:: Clone or update repository
if exist "rustalk-workflow" (
    echo [INFO] Directory exists, pulling latest changes...
    cd rustalk-workflow
    git pull
) else (
    echo [INFO] Cloning repository...
    git clone https://github.com/rustalk/rustalk-workflow.git
    cd rustalk-workflow
)

:: Build and install
echo [INFO] Building Rustalk packages...
cargo build --release

echo [INFO] Installing Rustalk binaries...
cargo install --path packages/rustalk --force
cargo install --path packages/rus --force

cd ..

echo [SUCCESS] Rust version installed successfully!
echo [INFO] Try: rustalk setup

if "%INSTALL_TYPE%"=="rust" goto :end
goto :install_js

:install_js
echo [INFO] Installing JavaScript version...

:: Check for Bun or Node.js
where /q bun
if not errorlevel 1 (
    set "RUNTIME=bun"
    echo [SUCCESS] Bun runtime detected
    goto :js_install_deps
)

where /q node
if not errorlevel 1 (
    set "RUNTIME=npm"
    echo [SUCCESS] Node.js runtime detected
    goto :js_install_deps
)

echo [ERROR] Neither Bun nor Node.js found.
echo Please install one of them:
echo   Bun: https://bun.sh/
echo   Node.js: https://nodejs.org/
pause
exit /b 1

:js_install_deps
:: Clone or update repository
if exist "rustalk-workflow" (
    echo [INFO] Directory exists, pulling latest changes...
    cd rustalk-workflow
    git pull
) else (
    echo [INFO] Cloning repository...
    git clone https://github.com/rustalk/rustalk-workflow.git
    cd rustalk-workflow
)

if "%RUNTIME%"=="bun" (
    echo [INFO] Installing dependencies with Bun...
    bun install
    
    echo [INFO] Building TypeScript...
    bun run build
    
    echo [INFO] Installing globally...
    npm install -g .
) else (
    echo [INFO] Installing dependencies with npm...
    npm install
    
    echo [INFO] Building TypeScript...
    npm run build
    
    echo [INFO] Installing globally...
    npm install -g .
)

cd ..

echo [SUCCESS] JavaScript version installed successfully!
echo [INFO] Try: rustalk setup
goto :end

:install_both
echo [INFO] Installing both Rust and JavaScript versions...
call :install_rust
call :install_js
echo [SUCCESS] Both versions installed successfully!
goto :end

:uninstall
echo [INFO] Uninstalling Rustalk...

:: Remove Rust binaries
where /q cargo
if not errorlevel 1 (
    cargo uninstall rustalk 2>nul
    cargo uninstall rus 2>nul
    echo [SUCCESS] Rust version uninstalled
)

:: Remove npm package
where /q npm
if not errorlevel 1 (
    npm uninstall -g rustalk 2>nul
    echo [SUCCESS] JavaScript version uninstalled
)

:: Remove source directory
if exist "rustalk-workflow" (
    set /p "REMOVE=Remove source directory? (y/N): "
    if /i "!REMOVE!"=="y" (
        rmdir /s /q rustalk-workflow
        echo [SUCCESS] Source directory removed
    )
)

echo [SUCCESS] Uninstall complete
goto :end

:help
echo Rustalk Installation Script for Windows
echo.
echo Usage: %0 [OPTIONS]
echo.
echo Options:
echo   --rust, -r     Install Rust version only
echo   --js, -j       Install JavaScript version only
echo   --both, -b     Install both versions (default)
echo   --uninstall    Uninstall Rustalk
echo   --help, -h     Show this help
echo.
echo Examples:
echo   %0              # Install both versions
echo   %0 --rust       # Install Rust version only
echo   %0 --js         # Install JavaScript version only
echo   %0 --uninstall  # Remove Rustalk
goto :end

:end
echo.
echo [SUCCESS] Installation complete!
echo.
echo 🚀 Next steps:
echo   1. Run: rustalk setup  
echo   2. Run: rustalk chat
echo   3. Share your connection info with friends!
echo.
echo 📖 For help: rustalk --help
echo 🐛 Report issues: https://github.com/rustalk/rustalk-workflow/issues
echo.
pause