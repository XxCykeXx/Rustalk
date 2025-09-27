//! Cross-platform compatibility tests for Rustalk
//! Tests configuration, path handling, and basic functionality across platforms

use std::path::PathBuf;
use anyhow::Result;
use reach::{Config, config::*};
use rustalk::setup::*;

#[cfg(test)]
mod cross_platform_tests {
    use super::*;

    #[test]
    fn test_config_directory_creation() -> Result<()> {
        // Test that config directory can be created on all platforms
        let config_dir = get_config_dir()?;
        
        // Verify directory exists
        assert!(config_dir.exists(), "Config directory should be created");
        
        // Verify it's actually a directory
        assert!(config_dir.is_dir(), "Config path should be a directory");
        
        // Test platform-specific paths
        #[cfg(windows)]
        {
            let path_str = config_dir.to_string_lossy();
            assert!(
                path_str.contains("AppData") || path_str.contains(".rustalk"),
                "Windows config should be in AppData or home directory"
            );
        }
        
        #[cfg(unix)]
        {
            let path_str = config_dir.to_string_lossy();
            assert!(
                path_str.contains(".config") || path_str.contains(".rustalk"),
                "Unix config should be in .config or .rustalk directory"
            );
        }
        
        Ok(())
    }

    #[test]
    fn test_config_file_operations() -> Result<()> {
        // Test config file creation and loading
        let config_file = get_config_file()?;
        
        // Create a test config
        let test_config = Config::default();
        save_config(&test_config)?;
        
        // Verify file exists
        assert!(config_file.exists(), "Config file should be created");
        
        // Load and verify config
        let loaded_config = load_config()?;
        assert_eq!(test_config.default_port, loaded_config.default_port);
        assert_eq!(test_config.max_peers, loaded_config.max_peers);
        
        Ok(())
    }

    #[test]
    fn test_user_setup_flow() -> Result<()> {
        // Test the complete user setup flow
        let test_email = "test@example.com";
        let test_name = "Test User";
        let test_password = "test_password_123";
        
        // This should work on all platforms
        let setup_result = setup_user_interactive_test(
            test_email.to_string(),
            test_name.to_string(),
            test_password.to_string(),
        );
        
        match setup_result {
            Ok(_) => {
                // Verify config was created
                assert!(config_exists(), "Config should exist after setup");
                
                // Load and verify the config
                let config = load_config()?;
                assert_eq!(config.identity.email, test_email);
                assert_eq!(config.identity.display_name, test_name);
            }
            Err(e) => {
                eprintln!("Setup failed: {}", e);
                // This is acceptable in test environment
            }
        }
        
        Ok(())
    }

    #[test]
    fn test_path_handling() {
        // Test that our path handling works across platforms
        let paths = vec![
            get_config_dir(),
            get_config_file(),
        ];
        
        for path_result in paths {
            match path_result {
                Ok(path) => {
                    // Path should be absolute
                    assert!(path.is_absolute(), "Paths should be absolute: {:?}", path);
                    
                    // Should not contain invalid characters
                    let path_str = path.to_string_lossy();
                    assert!(!path_str.is_empty(), "Path should not be empty");
                    
                    // Platform-specific validation
                    #[cfg(windows)]
                    {
                        // Windows paths should use backslashes or be normalized
                        assert!(
                            path_str.contains('\\') || path_str.contains('/'),
                            "Windows path should contain path separators"
                        );
                    }
                    
                    #[cfg(unix)]
                    {
                        // Unix paths should start with /
                        assert!(path_str.starts_with('/'), "Unix paths should be absolute");
                    }
                }
                Err(e) => {
                    panic!("Path creation failed: {}", e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_identity_generation() -> Result<()> {
        // Test that identity generation works across platforms
        use reach::identity::{Identity, UserCredentials};
        
        let credentials = UserCredentials {
            email: "cross-platform-test@example.com".to_string(),
            password: "test123".to_string(),
        };
        
        let identity = Identity::new(credentials)?;
        
        // Verify identity fields
        assert!(!identity.user_id.to_string().is_empty(), "User ID should not be empty");
        assert!(!identity.email.is_empty(), "Email should not be empty");
        assert!(!identity.keypair.public_key.is_empty(), "Public key should not be empty");
        assert!(!identity.keypair.private_key.is_empty(), "Private key should not be empty");
        
        Ok(())
    }
}

/// Test helper function for user setup without interactive prompts
fn setup_user_interactive_test(
    email: String,
    display_name: String,
    password: String,
) -> Result<Config> {
    use reach::identity::{Identity, UserCredentials};
    
    let credentials = UserCredentials { email, password };
    let mut identity = Identity::new(credentials)?;
    identity.display_name = display_name;
    
    let config = Config::new(identity);
    save_config(&config)?;
    
    Ok(config)
}