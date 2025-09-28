# Reach 🌐

**Core P2P Networking Library for Rustalk**

Reach is the foundational library that powers Rustalk's peer-to-peer communication capabilities. It provides secure, encrypted networking, identity management, and session handling.

## Features

- 🔐 **End-to-End Encryption**: AES-GCM encryption with secure key exchange
- 🌐 **P2P Networking**: Direct peer-to-peer communication without central servers
- 🔑 **Identity Management**: Secure user identity and credential handling
- 📡 **Session Management**: Robust session handling and peer discovery
- ⚡ **High Performance**: Built with Rust and Tokio for async performance
- 🛡️ **Security First**: Industry-standard cryptographic primitives

## Usage

Reach is designed to be used as a library by other Rustalk components:

```rust
use reach::{ReachEngine, UserCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = UserCredentials {
        email: "user@example.com".to_string(),
        password: "secure_password".to_string(),
    };
    
    let engine = ReachEngine::new(credentials).await?;
    engine.start_server(5000).await?;
    
    Ok(())
}
```

## Core Components

- **`ReachEngine`**: Main P2P engine orchestrating all components
- **`Identity`**: User identity and credential management
- **`NetworkManager`**: P2P networking and connection handling
- **`CryptoEngine`**: Encryption/decryption and key management
- **`SessionManager`**: Chat session and peer management
- **`Config`**: Configuration management and persistence

## Installation

Reach is typically used as a dependency in other Rust projects:

```toml
[dependencies]
reach = "0.0.1"
```

## License

Apache-2.0 License. See [LICENSE](../../LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for contribution guidelines.

---

Part of the [Rustalk](../../README.md) project.