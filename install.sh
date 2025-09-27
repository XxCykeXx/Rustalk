#!/bin/bash

# Rustalk Installation Script
# Supports both Rust and JavaScript installations

set -e

echo "🦀 Welcome to Rustalk Installation!"
echo "   Secure P2P Terminal Chat"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check system requirements
check_requirements() {
    print_status "Checking system requirements..."
    
    # Check for curl or wget
    if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
        print_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi
    
    # Check for git
    if ! command -v git &> /dev/null; then
        print_error "Git is required but not installed."
        exit 1
    fi
    
    print_success "System requirements met"
}

# Install Rust version
install_rust_version() {
    print_status "Installing Rust version..."
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_warning "Rust not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        print_success "Rust installed successfully"
    else
        print_success "Rust is already installed"
    fi
    
    # Install Rustalk
    print_status "Installing Rustalk from source..."
    
    if [ -d "rustalk-workflow" ]; then
        print_status "Directory exists, pulling latest changes..."
        cd rustalk-workflow
        git pull
    else
        git clone https://github.com/rustalk/rustalk-workflow.git
        cd rustalk-workflow
    fi
    
    # Build and install
    print_status "Building Rustalk packages..."
    cargo build --release
    
    print_status "Installing Rustalk binaries..."
    cargo install --path packages/rustalk --force
    cargo install --path packages/rus --force
    
    cd ..
    
    print_success "Rust version installed successfully!"
    print_status "Try: rustalk setup"
}

# Install JavaScript version
install_js_version() {
    print_status "Installing JavaScript version..."
    
    # Check for Node.js or Bun
    if command -v bun &> /dev/null; then
        RUNTIME="bun"
        print_success "Bun runtime detected"
    elif command -v node &> /dev/null; then
        RUNTIME="npm"
        print_success "Node.js runtime detected"
    else
        print_error "Neither Bun nor Node.js found."
        echo "Please install one of them:"
        echo "  Bun: curl -fsSL https://bun.sh/install | bash"
        echo "  Node.js: https://nodejs.org/"
        exit 1
    fi
    
    # Clone and build
    if [ -d "rustalk-workflow" ]; then
        print_status "Directory exists, pulling latest changes..."
        cd rustalk-workflow
        git pull
    else
        git clone https://github.com/rustalk/rustalk-workflow.git
        cd rustalk-workflow
    fi
    
    if [ "$RUNTIME" = "bun" ]; then
        print_status "Installing dependencies with Bun..."
        bun install
        
        print_status "Building TypeScript..."
        bun run build
        
        print_status "Installing globally..."
        npm install -g .
    else
        print_status "Installing dependencies with npm..."
        npm install
        
        print_status "Building TypeScript..."
        npm run build
        
        print_status "Installing globally..."
        npm install -g .
    fi
    
    cd ..
    
    print_success "JavaScript version installed successfully!"
    print_status "Try: rustalk setup"
}

# Install both versions
install_both() {
    print_status "Installing both Rust and JavaScript versions..."
    install_rust_version
    install_js_version
    print_success "Both versions installed successfully!"
}

# Uninstall function
uninstall() {
    print_status "Uninstalling Rustalk..."
    
    # Remove Rust binaries
    if command -v cargo &> /dev/null; then
        cargo uninstall rustalk 2>/dev/null || true
        cargo uninstall rus 2>/dev/null || true
        print_success "Rust version uninstalled"
    fi
    
    # Remove npm package
    if command -v npm &> /dev/null; then
        npm uninstall -g rustalk 2>/dev/null || true
        print_success "JavaScript version uninstalled"
    fi
    
    # Remove source directory
    if [ -d "rustalk-workflow" ]; then
        read -p "Remove source directory? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf rustalk-workflow
            print_success "Source directory removed"
        fi
    fi
    
    print_success "Uninstall complete"
}

# Show help
show_help() {
    echo "Rustalk Installation Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --rust, -r     Install Rust version only"
    echo "  --js, -j       Install JavaScript version only"
    echo "  --both, -b     Install both versions (default)"
    echo "  --uninstall    Uninstall Rustalk"
    echo "  --help, -h     Show this help"
    echo ""
    echo "Examples:"
    echo "  $0              # Install both versions"
    echo "  $0 --rust       # Install Rust version only"
    echo "  $0 --js         # Install JavaScript version only"
    echo "  $0 --uninstall  # Remove Rustalk"
}

# Main installation logic
main() {
    case "${1:-}" in
        --rust|-r)
            check_requirements
            install_rust_version
            ;;
        --js|-j)
            check_requirements
            install_js_version
            ;;
        --both|-b|"")
            check_requirements
            install_both
            ;;
        --uninstall)
            uninstall
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"

echo ""
print_success "Installation complete!"
echo ""
echo "🚀 Next steps:"
echo "  1. Run: rustalk setup"
echo "  2. Run: rustalk chat"
echo "  3. Share your connection info with friends!"
echo ""
echo "📖 For help: rustalk --help"
echo "🐛 Report issues: https://github.com/rustalk/rustalk-workflow/issues"