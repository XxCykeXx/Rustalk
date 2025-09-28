// install.rs - Post-install script for cargo install
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("🚀 Setting up Rustalk PATH integration...");
    
    if let Ok(cargo_home) = env::var("CARGO_HOME") {
        let bin_dir = PathBuf::from(cargo_home).join("bin");
        add_to_path(&bin_dir);
    } else if let Ok(home) = env::var("HOME") {
        let bin_dir = PathBuf::from(home).join(".cargo").join("bin");
        add_to_path(&bin_dir);
    } else {
        println!("⚠️  Could not determine cargo bin directory");
        println!("💡 Make sure ~/.cargo/bin is in your PATH");
    }
}

fn add_to_path(bin_dir: &PathBuf) {
    let bin_path = bin_dir.to_string_lossy();
    
    #[cfg(target_os = "windows")]
    {
        println!("🔧 Setting up Windows PATH...");
        
        // Check if cargo bin is already in PATH
        if let Ok(current_path) = env::var("PATH") {
            if current_path.contains(&*bin_path) {
                println!("✅ Cargo bin directory already in PATH");
                return;
            }
        }
        
        // Try to add to user PATH via PowerShell
        let ps_command = format!(
            r#"
            $oldPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
            $newPath = if ($oldPath) {{ "$oldPath;{}" }} else {{ "{}" }}
            [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')
            Write-Host 'PATH updated successfully'
            "#,
            bin_path.replace('\\', "\\\\"),
            bin_path.replace('\\', "\\\\")
        );
        
        match Command::new("powershell")
            .args(&["-Command", &ps_command])
            .output()
        {
            Ok(_) => {
                println!("✅ Added cargo bin to Windows PATH");
                println!("💡 Restart your terminal for PATH changes to take effect");
            }
            Err(e) => {
                println!("⚠️  Could not automatically update PATH: {}", e);
                println!("💡 Please manually add to your PATH: {}", bin_path);
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        println!("🔧 Setting up Unix PATH...");
        
        use std::fs::{File, OpenOptions};
        use std::io::{Read, Write, Seek, SeekFrom};
        
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let shell_profiles = vec![
            format!("{}/.bashrc", home),
            format!("{}/.zshrc", home),
            format!("{}/.profile", home),
        ];
        
        let path_export = format!(r#"export PATH="$PATH:{}""#, bin_path);
        let path_comment = "# Added by Rustalk cargo install";
        
        let mut updated = false;
        for profile in shell_profiles {
            if std::path::Path::new(&profile).exists() {
                if let Ok(mut file) = File::open(&profile) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() && !content.contains(&path_export) {
                        drop(file); // Close the read handle
                        
                        if let Ok(mut file) = OpenOptions::new().append(true).open(&profile) {
                            if writeln!(file, "\n{}\n{}", path_comment, path_export).is_ok() {
                                println!("✅ Added to {}", profile);
                                updated = true;
                            }
                        }
                    }
                }
            }
        }
        
        if updated {
            println!("✅ Added cargo bin to shell PATH");
            println!("💡 Run 'source ~/.bashrc' (or restart terminal) for changes to take effect");
        } else {
            println!("💡 Add this to your shell profile: {}", path_export);
        }
    }
    
    println!();
    println!("🎉 Rustalk installation complete!");
    println!("💡 You can now use 'rus' and 'rustalk_cli' commands from anywhere!");
}