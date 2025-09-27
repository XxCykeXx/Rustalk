use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use crate::crypto::{KeyPair, CryptoEngine};
use rand::{rngs::OsRng, RngCore};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub user_id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub keypair: KeyPairSerialized,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPairSerialized {
    pub private_key: String,
    pub public_key: String,
}

impl Identity {
    pub fn new(credentials: UserCredentials) -> Result<Self> {
        let user_id = Uuid::new_v4();
        let keypair = KeyPair::generate();
        
        // Generate salt for password hashing
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        
        let salted_password = format!("{}{}", credentials.password, hex::encode(&salt));
        let password_hash = CryptoEngine::hash_password(&salted_password);
        let password_hash_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD, 
            &password_hash
        );
        
        let keypair_serialized = KeyPairSerialized {
            private_key: base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &keypair.private_key
            ),
            public_key: keypair.public_key_base64(),
        };

        Ok(Identity {
            user_id,
            email: credentials.email,
            display_name: None,
            keypair: keypair_serialized,
            password_hash: password_hash_b64,
        })
    }

    pub fn from_existing(
        user_id: Uuid,
        email: String,
        display_name: Option<String>,
        keypair: KeyPairSerialized,
        password_hash: String,
    ) -> Self {
        Identity {
            user_id,
            email,
            display_name,
            keypair,
            password_hash,
        }
    }

    pub fn set_display_name(&mut self, name: String) {
        self.display_name = Some(name);
    }

    pub fn get_public_key_bytes(&self) -> Result<[u8; 32]> {
        let bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &self.keypair.public_key
        ).map_err(|e| anyhow!("Failed to decode public key: {}", e))?;
        
        if bytes.len() != 32 {
            return Err(anyhow!("Invalid public key length"));
        }
        
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        Ok(key_bytes)
    }

    pub fn get_private_key_bytes(&self) -> Result<[u8; 32]> {
        let bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &self.keypair.private_key
        ).map_err(|e| anyhow!("Failed to decode private key: {}", e))?;
        
        if bytes.len() != 32 {
            return Err(anyhow!("Invalid private key length"));
        }
        
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        Ok(key_bytes)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        // In a real implementation, you'd store the salt and verify properly
        // This is a simplified version
        let salt = [0u8; 16]; // You'd store this with the identity
        let salted_password = format!("{}{}", password, hex::encode(salt));
        let hash = CryptoEngine::hash_password(&salted_password);
        let hash_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD, 
            &hash
        );
        
        hash_b64 == self.password_hash
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
            .unwrap_or_else(|| format!("User_{}", &self.user_id.to_string()[..8]))
    }
}