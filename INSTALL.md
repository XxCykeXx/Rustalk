# Installing Rustalk with Automatic PATH Setup

## NPM Installation (Recommended)

```bash
# Global installation (automatically adds to PATH)
npm install -g rustalk

# After installation, both commands are available:
rustalk --help
rus --help
```

## Cargo Installation

```bash
# Install individual packages
cargo install reach  # Core library (for development)
cargo install rus     # CLI interface
cargo install rustalk # Main application

# After installation, ensure ~/.cargo/bin is in your PATH
```

### Manual PATH Setup (if needed)

#### Windows (PowerShell)
```powershell
# Add npm global bin to PATH
$env:PATH += ";$env:APPDATA\npm"

# Add cargo bin to PATH  
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Make permanent
[Environment]::SetEnvironmentVariable("PATH", $env:PATH, "User")
```

#### Unix/Linux/macOS
```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
echo 'export PATH="$PATH:~/.cargo/bin"' >> ~/.bashrc
echo 'export PATH="$PATH:$(npm config get prefix)/bin"' >> ~/.bashrc

# Reload shell
source ~/.bashrc
```

## Verification

After installation, verify the commands work:

```bash
# Check versions
rustalk --version
rus --version

# Quick setup
rustalk setup
rus setup

# Start using
rustalk chat
rus chat
```

## Troubleshooting

If commands are not found:

1. **NPM**: Ensure global npm bin directory is in PATH
2. **Cargo**: Ensure `~/.cargo/bin` is in PATH  
3. **Restart** your terminal/command prompt
4. **Log out/in** on Windows for system PATH changes

## Development Installation

For development with local paths:

```bash
git clone <repository>
cd rustalk-workflow
npm install
npm run build
```