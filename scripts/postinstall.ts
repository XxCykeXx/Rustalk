import { execSync } from 'child_process';
import { existsSync, mkdirSync, unlinkSync, copyFileSync, chmodSync } from 'fs';
import { join, dirname } from 'path';
import { platform, arch } from 'os';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

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