# Contributing to Rustalk

Welcome to Rustalk! We're excited that you're interested in contributing to our modular P2P communication platform. This guide will help you get started with contributing to this project.

## Table of Contents

- [About Rustalk](#about-rustalk)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Building the Project](#building-the-project)
- [Running Tests](#running-tests)
- [Code Style and Standards](#code-style-and-standards)
- [Making Changes](#making-changes)
- [Submitting Pull Requests](#submitting-pull-requests)
- [Reporting Issues](#reporting-issues)

## About Rustalk

Rustalk is a modular, secure peer-to-peer communication platform with three main components:

- **reach** - Core P2P networking library with end-to-end encryption
- **rus** - CLI operations and user management interface  
- **rustalk** - Binary installer and starter with npm integration

### Key Features

- **End-to-end encryption** using AES-GCM with SHA2-based key derivation
- **P2P networking** with async Tokio runtime
- **Cross-platform support** (Windows, Linux, macOS)
- **Dual installation** methods (Cargo and npm)
- **Modular architecture** allowing independent use of components
- **Clean dependency hierarchy** (reach ← rus ← rustalk)

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable version) - [Install Rust](https://rustup.rs/)
- **Node.js** (v16 or later) - [Install Node.js](https://nodejs.org/)
- **Bun** (latest version) - [Install Bun](https://bun.sh/)
- **Git** - [Install Git](https://git-scm.com/)

### Quick Start

1. **Clone the repository:**
   ```bash
   git clone https://github.com/muhammad-fiaz/Rustalk.git
   cd Rustalk
   ```

2. **Install dependencies:**
   ```bash
   bun install
   ```

3. **Build the project:**
   ```bash
   bun run src/index.ts build
   ```

4. **Run tests:**
   ```bash
   bun run src/index.ts test
   ```

## Development Setup

### Environment Setup

1. **Configure Rust toolchain:**
   ```bash
   rustup update stable
   rustup default stable
   ```

2. **Install required Rust components:**
   ```bash
   rustup component add rustfmt clippy
   ```

3. **Verify installation:**
   ```bash
   cargo --version
   rustc --version
   bun --version
   ```

### IDE Setup

We recommend using **VS Code** with the following extensions:
- Rust Analyzer
- Better TOML
- TypeScript Importer

## Project Structure

```
rustalk-workflow/
├── packages/                   # Library packages
│   ├── reach/                 # Core P2P networking library
│   │   ├── src/
│   │   │   ├── lib.rs         # Library entry point
│   │   │   ├── config.rs      # Configuration management
│   │   │   ├── crypto.rs      # Encryption/decryption
│   │   │   ├── identity.rs    # User identity management
│   │   │   ├── network.rs     # P2P networking
│   │   │   ├── peer.rs        # Peer connection management
│   │   │   └── session.rs     # Session management
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── LICENSE
│   └── rus/                   # CLI operations library + binary
│       ├── src/
│       │   ├── lib.rs         # Library entry point
│       │   ├── main.rs        # CLI binary
│       │   ├── path_manager.rs # System PATH management
│       │   └── user_manager.rs # User management
│       ├── Cargo.toml
│       ├── README.md
│       └── LICENSE
├── rustalk/                   # Main binary package
│   ├── src/
│   │   ├── main.rs           # Installer/starter binary
│   │   └── lib.rs            # Library for npm integration
│   ├── Cargo.toml
│   ├── README.md
│   └── LICENSE
├── src/                       # TypeScript npm wrapper
│   ├── index.ts              # Main npm entry point
│   └── types.ts              # TypeScript definitions
├── bin/                       # CLI scripts
│   └── rustalk.ts            # npm CLI script
├── tests/                     # Test suite
│   ├── cross_platform.test.ts
│   ├── cli.test.ts
│   └── integration_test.rs
├── target/                    # Rust build artifacts
├── dist/                      # TypeScript build output
├── Cargo.toml                # Workspace configuration
├── package.json              # Node.js package config
├── tsconfig.json             # TypeScript configuration
├── README.md                 # Main project documentation
├── CONTRIBUTING.md           # This file
├── LICENSE                   # Apache 2.0 license
└── bun.lock                  # Bun lockfile
```

## Building the Project

### Build Commands

1. **Full build (Rust + TypeScript):**
   ```bash
   bun run build
   ```

2. **Rust workspace (release):**
   ```bash
   cargo build --release --workspace
   ```

3. **Rust workspace (debug):**
   ```bash
   cargo build --workspace
   ```

4. **TypeScript only:**
   ```bash
   bun run build:ts
   ```

5. **Clean build:**
   ```bash
   bun run clean
   bun run build
   ```

### Available Binaries

- **rustalk_cli**: Main installer/starter binary
- **rus**: CLI operations binary  
- **reach**: Library only (no binary)

### Package Types

- **reach**: Pure library package (`rlib`)
- **rus**: Library + binary package (`rlib` + binary)
- **rustalk**: Primary binary + npm library (`cdylib`, `rlib` + binary)

## Running Tests

### Test Commands

1. **All tests:**
   ```bash
   bun run test
   ```

2. **Rust workspace tests:**
   ```bash
   cargo test --workspace
   ```

3. **TypeScript tests only:**
   ```bash
   bun test tests/**/*.test.ts
   ```

4. **Specific package tests:**
   ```bash
   cargo test -p reach
   cargo test -p rus
   cargo test -p rustalk
   ```

5. **Integration tests:**
   ```bash
   bun run test:cli
   ```

### Test Coverage

Our test suite includes:
- **Cross-platform compatibility tests**
- **CLI functionality tests**
- **Integration tests**
- **Configuration tests**
- **Path handling tests**

## Code Style and Standards

### Rust Code Style

- Follow **Rust standard style** using `rustfmt`
- Use `clippy` for linting: `cargo clippy`
- Write comprehensive documentation with `///` comments
- Use `#[allow(dead_code)]` for code intended for future development
- Prefer explicit error handling with `Result<T, E>`

### TypeScript Code Style

- Use **ESLint** and **Prettier** (configured in project)
- Use explicit types where beneficial
- Follow async/await patterns
- Keep CLI logic minimal - delegate to Rust binary

### Commit Messages

Use conventional commit format:
```
type(scope): description

Examples:
feat(cli): add status command for peer checking
fix(crypto): resolve key derivation issue
docs(readme): update installation instructions
test(cross-platform): add Windows path tests
```

## Making Changes

### Development Workflow

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**

3. **Test your changes:**
   ```bash
   bun run src/index.ts build
   bun run src/index.ts test
   ```

4. **Format your code:**
   ```bash
   cargo fmt
   cargo clippy
   ```

5. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat(component): your change description"
   ```

### Key Development Areas

- **Core P2P Logic**: Located in `packages/reach/src/`
- **CLI Interface**: Located in `src/index.ts`
- **Application Logic**: Located in `rustalk/src/app.rs`
- **Configuration**: Located in `packages/reach/src/config.rs`
- **Crypto**: Located in `packages/reach/src/crypto.rs`

## Submitting Pull Requests

1. **Ensure all tests pass**
2. **Update documentation if needed**
3. **Add tests for new functionality**
4. **Create a pull request with:**
   - Clear description of changes
   - Reference to any related issues
   - Test results

## Reporting Issues

When reporting issues, please include:

- **Operating system** and version
- **Rust version** (`rustc --version`)
- **Node.js/Bun version**
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Error messages** (full stack traces)

## Package Development Guidelines

### Adding Features to Packages

1. **reach (Core Library)**:
   - Implement core P2P functionality, encryption, networking
   - All changes should be library-focused (no main.rs)
   - Export functionality through `lib.rs`

2. **rus (CLI Operations)**:
   - Implement user-facing CLI commands and operations
   - Can be used as both library and binary
   - Imports and uses `reach` functionality

3. **rustalk (Installer/Starter)**:
   - Primarily binary package for installation and quick start
   - Delegates operations to `rus` for efficiency
   - Imports both `reach` and `rus` for library usage

### Code Organization

- **Modular Design**: Each package has a specific responsibility
- **Clean Dependencies**: reach ← rus ← rustalk (no circular dependencies)
- **Library First**: Prefer library functions over subprocess calls
- **Cross-Platform**: Support Windows, Linux, and macOS equally

### Security Best Practices

- Use Apache 2.0 license consistently across all packages
- Never store passwords in plaintext
- Use secure random number generation
- Validate all inputs thoroughly
- Use constant-time comparison for sensitive data

## Getting Help

- **Documentation**: Check existing docs and comments
- **Issues**: Search existing issues before creating new ones
- **Code**: Look at similar implementations in the codebase
- **Tests**: Check test files for usage examples

## Thank You

Thank you for contributing to Rustalk! Your efforts help make secure P2P communication accessible to everyone.