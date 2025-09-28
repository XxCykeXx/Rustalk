import { execSync } from 'child_process';
import { existsSync, mkdirSync, unlinkSync, copyFileSync, chmodSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { platform, arch, homedir } from 'os';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Function to add binaries to PATH
function addToPath() {
  const binDir = join(__dirname, '..', 'bin');
  
  try {
    if (currentPlatform === 'win32') {
      // Windows: Add to user PATH via registry
      console.log('🔧 Adding Rustalk binaries to Windows PATH...');
      
      // Check if already in PATH
      const currentPath = process.env.PATH || '';
      if (currentPath.includes(binDir)) {
        console.log('✅ Rustalk binaries already in PATH');
        return;
      }
      
      // Add to user PATH using PowerShell
      const psCommand = `
        $oldPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
        $newPath = if ($oldPath) { "$oldPath;${binDir.replace(/\\/g, '\\\\')}" } else { "${binDir.replace(/\\/g, '\\\\')}" }
        [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')
        Write-Host 'PATH updated successfully'
      `;
      
      execSync(`powershell -Command "${psCommand}"`, { stdio: 'pipe' });
      console.log('✅ Rustalk binaries added to Windows PATH');
      console.log('💡 Restart your terminal or log out/in for PATH changes to take effect');
      
    } else {
      // Unix-like systems: Add to shell profile
      console.log('🔧 Adding Rustalk binaries to Unix PATH...');
      
      const shellProfiles = [
        join(homedir(), '.bashrc'),
        join(homedir(), '.zshrc'),
        join(homedir(), '.profile')
      ];
      
      const pathExport = `export PATH="$PATH:${binDir}"`;
      const pathComment = '# Added by Rustalk npm package';
      
      let updated = false;
      for (const profile of shellProfiles) {
        if (existsSync(profile)) {
          try {
            const content = require('fs').readFileSync(profile, 'utf8');
            if (!content.includes(pathExport)) {
              require('fs').appendFileSync(profile, `\\n${pathComment}\\n${pathExport}\\n`);
              console.log(`✅ Added to ${profile}`);
              updated = true;
            }
          } catch (err) {
            console.log(`⚠️  Could not update ${profile}: ${(err as Error).message}`);
          }
        }
      }
      
      if (updated) {
        console.log('✅ Rustalk binaries added to shell PATH');
        console.log('💡 Run "source ~/.bashrc" (or your shell profile) or restart terminal');
      } else {
        console.log('💡 No shell profiles found. Add this to your shell profile:');
        console.log(`   ${pathExport}`);
      }
    }
    
  } catch (error) {
    console.log('⚠️  Could not automatically add to PATH:', (error as Error).message);
    console.log('💡 Manual PATH setup:');
    console.log(`   Add this directory to your PATH: ${binDir}`);
  }
}

console.log('🚀 Setting up Rustalk binaries...');

const currentPlatform = platform();
const currentArch = arch();

// Determine binary names based on platform
interface BinaryConfig {
  rus: string;
  rustalk_cli: string;
}

const binaries = {
  win32: {
    rus: 'rus.exe',
    rustalk_cli: 'rustalk_cli.exe'
  },
  darwin: {
    rus: 'rus',
    rustalk_cli: 'rustalk_cli'
  },
  linux: {
    rus: 'rus', 
    rustalk_cli: 'rustalk_cli'
  }
} as const;

type PlatformKey = keyof typeof binaries;
const platformKey: PlatformKey = currentPlatform === 'win32' ? 'win32' : currentPlatform === 'darwin' ? 'darwin' : 'linux';
const targetBinaries = binaries[platformKey];

// Create symlinks or copies for rus binary to be accessible
const projectRoot = dirname(__dirname);
const targetDir = join(projectRoot, 'target', 'release');
const binDir = join(projectRoot, 'bin');

// Ensure bin directory exists
if (!existsSync(binDir)) {
  mkdirSync(binDir, { recursive: true });
}

try {
  // Copy or link rus binary to bin directory with proper name
  const rusSrc = join(targetDir, targetBinaries.rus);
  const rusDest = join(binDir, currentPlatform === 'win32' ? 'rus.exe' : 'rus');
  
  if (existsSync(rusSrc)) {
    if (existsSync(rusDest)) {
      unlinkSync(rusDest);
    }
    copyFileSync(rusSrc, rusDest);
    
    // Make executable on Unix-like systems
    if (currentPlatform !== 'win32') {
      chmodSync(rusDest, '755');
    }
    
    console.log('✅ rus binary installed successfully');
  } else {
    console.log('⚠️  rus binary not found, will be built on first use');
  }
  
} catch (error) {
  console.log('⚠️  Binary setup completed with warnings:', (error as Error).message);
}

// Add binaries to PATH
addToPath();

console.log('');
console.log('🎉 Rustalk installation complete!');
console.log('');
console.log('🚀 TypeScript-powered P2P chat platform');
console.log('');
console.log('📋 Quick start:');
console.log('  rustalk setup    - Configure your identity');
console.log('  rustalk chat     - Start P2P chat session');
console.log('  rustalk --help   - Full command reference'); 
console.log('');
console.log('💡 After setup, you can also use "rus" for direct CLI access.');
console.log('🌐 Both rustalk (TypeScript) and rus (Rust) provide the same functionality!');
console.log('');
console.log('🛠️  Installation Methods:');
console.log('   npm install -g rustalk     # Global npm install (includes PATH setup)');
console.log('   cargo install rus          # Install rus CLI directly');  
console.log('   cargo install rustalk      # Install rustalk CLI directly');
console.log('');
console.log('📍 Binary Locations:');
if (currentPlatform === 'win32') {
  console.log('   npm: %APPDATA%\\npm (or global node_modules)');
  console.log('   cargo: %USERPROFILE%\\.cargo\\bin');
} else {
  console.log('   npm: /usr/local/lib/node_modules (or ~/.npm-global)');
  console.log('   cargo: ~/.cargo/bin');
}