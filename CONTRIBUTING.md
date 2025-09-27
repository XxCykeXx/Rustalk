# Contributing to Rustalk

Welcome to Rustalk! We're excited that you're interested in contributing to our secure P2P terminal chat application. This guide will help you get started with contributing to this project.

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
- [Architecture Guidelines](#architecture-guidelines)

## About Rustalk

Rustalk is a secure, peer-to-peer terminal chat application built with Rust for the core functionality and TypeScript for the CLI interface. The application features:

- **End-to-end encryption** using AES-GCM with SHA2-based key derivation
- **P2P networking** with async Tokio runtime
- **Cross-platform support** (Windows, Linux, macOS)
- **Native Node.js bindings** via NAPI for npm integration
- **Online status checking** before sending messages
- **Minimal TypeScript CLI** that delegates to Rust binary

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
├── src/                    # TypeScript CLI source
│   ├── index.ts           # Main CLI entry point
│   └── types.ts           # TypeScript type definitions
├── rustalk/               # Main Rust application (moved from packages)
│   ├── src/
│   │   ├── main.rs        # CLI binary entry point
│   │   ├── lib.rs         # NAPI bindings for Node.js
│   │   ├── app.rs         # Core application logic
│   │   ├── setup.rs       # User setup and configuration
│   │   └── ui.rs          # Terminal UI components
│   └── Cargo.toml
├── packages/
│   ├── reach/             # Core P2P networking library
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── config.rs  # Configuration management
│   │   │   ├── crypto.rs  # Encryption/decryption
│   │   │   ├── identity.rs # User identity management
│   │   │   ├── network.rs # P2P networking
│   │   │   └── peer.rs    # Peer connection management
│   │   └── Cargo.toml
│   └── rus/               # Additional CLI utilities
│       └── src/main.rs
├── tests/                 # Test suite
│   ├── cross_platform.test.ts
│   ├── cli.test.ts
│   └── integration_test.rs
├── target/                # Rust build artifacts
├── Cargo.toml            # Workspace configuration
├── package.json          # Node.js dependencies
├── tsconfig.json         # TypeScript configuration
└── bun.lock              # Bun lockfile
```

## Building the Project

### Build Commands

1. **Full build (Rust + TypeScript):**
   ```bash
   bun run src/index.ts build
   ```

2. **Rust only (release):**
   ```bash
   cargo build --release
   ```

3. **Rust only (debug):**
   ```bash
   cargo build
   ```

4. **Clean build:**
   ```bash
   cargo clean
   bun run src/index.ts build
   ```

### Build Targets

- **rustalk_cli**: Main CLI binary (`target/release/rustalk_cli.exe` on Windows)
- **rustalk_lib**: NAPI library for Node.js integration
- **reach**: Core P2P networking library
- **rus**: Additional utilities

## Running Tests

### Test Commands

1. **All tests:**
   ```bash
   bun run src/index.ts test
   ```

2. **Rust tests only:**
   ```bash
   cargo test
   ```

3. **TypeScript tests only:**
   ```bash
   bun test
   ```

4. **Specific test file:**
   ```bash
   bun test tests/cross_platform.test.ts
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

## Architecture Guidelines

### Design Principles

1. **Rust Core, TypeScript Wrapper**: All business logic in Rust, minimal TypeScript CLI
2. **Native CPU Bindings**: Use NAPI, not WASM, for Node.js integration
3. **Cross-Platform First**: Support Windows, Linux, and macOS equally
4. **Security by Design**: End-to-end encryption, secure key management
5. **Async Everything**: Use Tokio for all I/O operations

### Adding New Features

1. **Core Logic**: Implement in Rust (`rustalk/src/` or `packages/reach/src/`)
2. **CLI Interface**: Add command in TypeScript (`src/index.ts`)
3. **NAPI Bindings**: Expose functionality via `rustalk/src/lib.rs`
4. **Tests**: Add tests in `tests/` directory
5. **Documentation**: Update this file and relevant docs

### Performance Considerations

- Use `Arc<RwLock<T>>` for shared state
- Prefer `async/await` over blocking operations
- Use `serde` for efficient serialization
- Keep TypeScript CLI lightweight

### Security Considerations

- Never store passwords in plaintext
- Use secure random number generation
- Validate all inputs
- Use constant-time comparison for sensitive data

## Getting Help

- **Documentation**: Check existing docs and comments
- **Issues**: Search existing issues before creating new ones
- **Code**: Look at similar implementations in the codebase
- **Tests**: Check test files for usage examples

## Thank You

Thank you for contributing to Rustalk! Your efforts help make secure P2P communication accessible to everyone.