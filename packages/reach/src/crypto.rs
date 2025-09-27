use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit}};
use rand::{rngs::OsRng, RngCore};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use hex;

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub private_key: [u8; 32],
    pub public_key: [u8; 32],
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut private_key = [0u8; 32];
        OsRng.fill_bytes(&mut private_key);
        
        // For simplicity, derive public key from private key using SHA256
        let mut hasher = Sha256::new();
        hasher.update(&private_key);
        let public_key: [u8; 32] = hasher.finalize().into();
        
        KeyPair {
            private_key,
            public_key,
        }
    }

    pub fn from_private_key(private_key: [u8; 32]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&private_key);
        let public_key: [u8; 32] = hasher.finalize().into();
        
        KeyPair {
            private_key,
            public_key,
        }
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key)
    }

    pub fn private_key_hex(&self) -> String {
        hex::encode(self.private_key)
    }

    pub fn public_key_base64(&self) -> String {
        general_purpose::STANDARD.encode(self.public_key)
    }

    pub fn private_key_base64(&self) -> String {
        general_purpose::STANDARD.encode(self.private_key)
    }
}

pub struct CryptoEngine;

impl CryptoEngine {
    pub fn new() -> Self {
        CryptoEngine
    }

    pub fn generate_keypair() -> KeyPair {
        KeyPair::generate()
    }

    pub fn generate_shared_secret(our_private: &[u8; 32], their_public: &[u8; 32]) -> [u8; 32] {
        // Simple shared secret generation using XOR and hash
        // In production, use proper ECDH
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(our_private);
        combined[32..].copy_from_slice(their_public);
        
        let mut hasher = Sha256::new();
        hasher.update(&combined);
        hasher.finalize().into()
    }

    pub fn encrypt_message(message: &str, shared_secret: &[u8; 32]) -> Result<String> {
        let key = Key::<Aes256Gcm>::from_slice(shared_secret);
        let cipher = Aes256Gcm::new(key);
        
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher
            .encrypt(nonce, message.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        // Combine nonce and ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(general_purpose::STANDARD.encode(result))
    }

    pub fn decrypt_message(encrypted_message: &str, shared_secret: &[u8; 32]) -> Result<String> {
        let encrypted_data = general_purpose::STANDARD
            .decode(encrypted_message)
            .map_err(|e| anyhow!("Base64 decode failed: {}", e))?;
        
        if encrypted_data.len() < 12 {
            return Err(anyhow!("Invalid encrypted message length"));
        }
        
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let key = Key::<Aes256Gcm>::from_slice(shared_secret);
        let cipher = Aes256Gcm::new(key);
        
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| anyhow!("UTF-8 decode failed: {}", e))
    }

    pub fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hex::encode(hasher.finalize())
    }
}