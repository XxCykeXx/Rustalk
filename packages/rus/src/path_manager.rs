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

pub fn add_to_path() -> Result<()> {
    PathManager::new()?.add_to_path()
}

pub fn remove_from_path() -> Result<()> {
    PathManager::new()?.remove_from_path()
}

pub fn check_in_path() -> Result<bool> {
    Ok(PathManager::new()?.is_in_path())
}

pub fn get_path_status() -> Result<()> {
    let manager = PathManager::new()?;
    let in_path = manager.is_in_path();

    if in_path {
        println!("✅ rustalk is in PATH");
        println!("💡 You can run 'rustalk' from anywhere in your terminal");
    } else {
        println!("❌ rustalk is NOT in PATH");
        println!("💡 Run 'rus path add' to add it to your PATH");
    }

    Ok(())
}

impl PathManager {
    pub fn new() -> Result<Self> {
        let current_exe = env::current_exe()
            .map_err(|e| anyhow!("Failed to get current executable path: {}", e))?;

        let binary_path = current_exe
            .parent()
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
            path_var
                .split(path_separator)
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

        let output = Command::new("powershell")
            .creation_flags(0x08000000)
            .args(&[
                "-Command",
                &format!(
                    "$env:PATH = [Environment]::GetEnvironmentVariable('PATH', 'User'); \
                     if ($env:PATH -notlike '*{}*') {{ \
                         [Environment]::SetEnvironmentVariable('PATH', $env:PATH + ';{}', 'User') \
                     }}",
                    path_str, path_str
                ),
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute PowerShell command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to add to PATH: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("✅ Added to Windows PATH (User environment)");
        println!("🔄 Please restart your terminal or run 'refreshenv' to apply changes");
        Ok(())
    }

    #[cfg(not(windows))]
    fn add_to_path_windows(&self) -> Result<()> {
        Err(anyhow!(
            "Windows PATH operations not supported on this platform"
        ))
    }

    fn add_to_path_unix(&self) -> Result<()> {
        let home_dir =
            env::var("HOME").map_err(|_| anyhow!("HOME environment variable not set"))?;

        let shell_files = self.platform.shell_config_files();
        let path_str = self.binary_path.to_string_lossy();
        let export_line = format!("export PATH=\"{}:$PATH\"", path_str);

        let mut updated_files = Vec::new();

        for shell_file in shell_files {
            let shell_path = Path::new(&home_dir).join(shell_file);

            if shell_path.exists() {
                let content = fs::read_to_string(&shell_path)
                    .map_err(|e| anyhow!("Failed to read {}: {}", shell_file, e))?;

                if !content.contains(&*path_str) {
                    let mut new_content = content;
                    if !new_content.ends_with('\n') {
                        new_content.push('\n');
                    }
                    new_content.push_str(&format!("# Added by rustalk\n{}\n", export_line));

                    fs::write(&shell_path, new_content)
                        .map_err(|e| anyhow!("Failed to write to {}: {}", shell_file, e))?;

                    updated_files.push(shell_file);
                }
            }
        }

        if updated_files.is_empty() {
            println!("✅ Already in PATH or no shell config files found");
        } else {
            println!("✅ Added to PATH in: {}", updated_files.join(", "));
            println!("🔄 Please run 'source ~/.bashrc' or restart your terminal to apply changes");
        }

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
            .creation_flags(0x08000000)
            .args(&[
                "-Command",
                &format!(
                    "$currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User'); \
                     $newPath = ($currentPath -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'; \
                     [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')",
                    path_str
                )
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute PowerShell command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to remove from PATH: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("✅ Removed from Windows PATH");
        println!("🔄 Please restart your terminal to apply changes");
        Ok(())
    }

    #[cfg(not(windows))]
    fn remove_from_path_windows(&self) -> Result<()> {
        Err(anyhow!(
            "Windows PATH operations not supported on this platform"
        ))
    }

    fn remove_from_path_unix(&self) -> Result<()> {
        let home_dir =
            env::var("HOME").map_err(|_| anyhow!("HOME environment variable not set"))?;

        let shell_files = self.platform.shell_config_files();
        let path_str = self.binary_path.to_string_lossy();

        let mut updated_files = Vec::new();

        for shell_file in shell_files {
            let shell_path = Path::new(&home_dir).join(shell_file);

            if shell_path.exists() {
                let content = fs::read_to_string(&shell_path)
                    .map_err(|e| anyhow!("Failed to read {}: {}", shell_file, e))?;

                let lines: Vec<&str> = content.lines().collect();
                let mut new_lines = Vec::new();
                let mut skip_next = false;

                for line in lines {
                    if skip_next && line.contains(&*path_str) {
                        skip_next = false;
                        continue;
                    }

                    if line.trim() == "# Added by rustalk" {
                        skip_next = true;
                        continue;
                    }

                    if !line.contains(&*path_str) {
                        new_lines.push(line);
                    }

                    skip_next = false;
                }

                let new_content = new_lines.join("\n");
                if new_content != content {
                    fs::write(&shell_path, new_content)
                        .map_err(|e| anyhow!("Failed to write to {}: {}", shell_file, e))?;
                    updated_files.push(shell_file);
                }
            }
        }

        if updated_files.is_empty() {
            println!("✅ Not found in PATH or already removed");
        } else {
            println!("✅ Removed from PATH in: {}", updated_files.join(", "));
            println!("🔄 Please run 'source ~/.bashrc' or restart your terminal to apply changes");
        }

        Ok(())
    }
}
