use anyhow::{Result, anyhow};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub struct PathManager {
    binary_path: PathBuf,
    platform: Platform,
}

#[derive(Debug, Clone)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
}

impl Platform {
    pub fn current() -> Platform {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else {
            Platform::Linux
        }
    }

    pub fn shell_config_files(&self) -> Vec<&'static str> {
        match self {
            Platform::Windows => vec![],
            Platform::Linux => vec![".bashrc", ".zshrc", ".profile", ".bash_profile"],
            Platform::MacOS => vec![".zshrc", ".bash_profile", ".profile"],
        }
    }
}

impl PathManager {
    pub fn new() -> Result<Self> {
        let current_exe = env::current_exe()
            .map_err(|e| anyhow!("Failed to get current executable path: {}", e))?;
        
        let binary_path = current_exe.parent()
            .ok_or_else(|| anyhow!("Failed to get parent directory of executable"))?
            .to_path_buf();

        Ok(PathManager {
            binary_path,
            platform: Platform::current(),
        })
    }

    pub fn is_in_path(&self) -> bool {
        if let Ok(path_var) = env::var("PATH") {
            let path_separator = if cfg!(windows) { ';' } else { ':' };
            path_var.split(path_separator)
                .any(|path| Path::new(path) == self.binary_path)
        } else {
            false
        }
    }

    pub fn add_to_path(&self) -> Result<()> {
        match self.platform {
            Platform::Windows => self.add_to_path_windows(),
            Platform::Linux | Platform::MacOS => self.add_to_path_unix(),
        }
    }

    #[cfg(windows)]
    fn add_to_path_windows(&self) -> Result<()> {
        let path_str = self.binary_path.to_string_lossy();
        
        // Add to user PATH using registry
        let output = Command::new("powershell")
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .args(&[
                "-Command",
                &format!(
                    "$env:PATH = [Environment]::GetEnvironmentVariable('PATH', 'User'); \
                     if ($env:PATH -notlike '*{}*') {{ \
                         [Environment]::SetEnvironmentVariable('PATH', $env:PATH + ';{}', 'User') \
                     }}",
                    path_str, path_str
                )
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute PowerShell command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!("Failed to add to PATH: {}", String::from_utf8_lossy(&output.stderr)));
        }

        println!("✅ Added to Windows PATH (User environment)");
        println!("🔄 Please restart your terminal or run 'refreshenv' to apply changes");
        Ok(())
    }

    #[cfg(not(windows))]
    fn add_to_path_windows(&self) -> Result<()> {
        Err(anyhow!("Windows PATH management not available on this platform"))
    }

    fn add_to_path_unix(&self) -> Result<()> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not find home directory"))?;

        let path_str = self.binary_path.to_string_lossy();
        let export_line = format!("export PATH=\"{}:$PATH\"", path_str);

        let config_files = self.platform.shell_config_files();
        let mut updated_files = Vec::new();

        for config_file in config_files {
            let config_path = home_dir.join(config_file);
            
            if config_path.exists() {
                let contents = fs::read_to_string(&config_path)
                    .map_err(|e| anyhow!("Failed to read {}: {}", config_file, e))?;

                if !contents.contains(&path_str.to_string()) {
                    let new_contents = format!("{}\n# Rustalk PATH\n{}\n", contents.trim(), export_line);
                    fs::write(&config_path, new_contents)
                        .map_err(|e| anyhow!("Failed to write to {}: {}", config_file, e))?;
                    updated_files.push(config_file);
                }
            } else if config_file == ".profile" || config_file == ".bash_profile" {
                // Create the file if it's a common one
                fs::write(&config_path, format!("# Rustalk PATH\n{}\n", export_line))
                    .map_err(|e| anyhow!("Failed to create {}: {}", config_file, e))?;
                updated_files.push(config_file);
                break; // Only create one file
            }
        }

        if updated_files.is_empty() {
            return Err(anyhow!("No suitable shell configuration file found"));
        }

        println!("✅ Added to PATH in: {}", updated_files.join(", "));
        println!("🔄 Please restart your terminal or run 'source ~/.profile' to apply changes");
        Ok(())
    }

    pub fn remove_from_path(&self) -> Result<()> {
        match self.platform {
            Platform::Windows => self.remove_from_path_windows(),
            Platform::Linux | Platform::MacOS => self.remove_from_path_unix(),
        }
    }

    #[cfg(windows)]
    fn remove_from_path_windows(&self) -> Result<()> {
        let path_str = self.binary_path.to_string_lossy();
        
        let output = Command::new("powershell")
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .args(&[
                "-Command",
                &format!(
                    "$env:PATH = [Environment]::GetEnvironmentVariable('PATH', 'User'); \
                     $env:PATH = $env:PATH -replace ';{}', '' -replace '{};', '' -replace '{}', ''; \
                     [Environment]::SetEnvironmentVariable('PATH', $env:PATH, 'User')",
                    path_str, path_str, path_str
                )
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute PowerShell command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!("Failed to remove from PATH: {}", String::from_utf8_lossy(&output.stderr)));
        }

        println!("✅ Removed from Windows PATH");
        println!("🔄 Please restart your terminal to apply changes");
        Ok(())
    }

    #[cfg(not(windows))]
    fn remove_from_path_windows(&self) -> Result<()> {
        Err(anyhow!("Windows PATH management not available on this platform"))
    }

    fn remove_from_path_unix(&self) -> Result<()> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not find home directory"))?;

        let path_str = self.binary_path.to_string_lossy();
        let config_files = self.platform.shell_config_files();
        let mut updated_files = Vec::new();

        for config_file in config_files {
            let config_path = home_dir.join(config_file);
            
            if config_path.exists() {
                let contents = fs::read_to_string(&config_path)
                    .map_err(|e| anyhow!("Failed to read {}: {}", config_file, e))?;

                if contents.contains(&path_str.to_string()) {
                    let lines: Vec<&str> = contents.lines().collect();
                    let filtered_lines: Vec<&str> = lines.into_iter()
                        .filter(|line| !line.contains(&path_str.to_string()) && !line.contains("# Rustalk PATH"))
                        .collect();
                    
                    let new_contents = filtered_lines.join("\n");
                    fs::write(&config_path, new_contents)
                        .map_err(|e| anyhow!("Failed to write to {}: {}", config_file, e))?;
                    updated_files.push(config_file);
                }
            }
        }

        if updated_files.is_empty() {
            println!("ℹ️  Rustalk was not found in PATH configuration files");
        } else {
            println!("✅ Removed from PATH in: {}", updated_files.join(", "));
            println!("🔄 Please restart your terminal to apply changes");
        }

        Ok(())
    }

    pub fn get_install_location(&self) -> &PathBuf {
        &self.binary_path
    }

    pub fn get_platform(&self) -> &Platform {
        &self.platform
    }
}