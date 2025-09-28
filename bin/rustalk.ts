#!/usr/bin/env node

import { spawn } from 'child_process';
import { existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { platform } from 'os';

const __dirname = dirname(fileURLToPath(import.meta.url));
const currentPlatform = platform();

function findRustalkBinary(): string {
  const possiblePaths = [
    // From compiled dist (when installed via npm)
    join(__dirname, '..', '..', 'target', 'release', currentPlatform === 'win32' ? 'rustalk.exe' : 'rustalk'),
    // From target debug (development)
    join(__dirname, '..', '..', 'target', 'debug', currentPlatform === 'win32' ? 'rustalk.exe' : 'rustalk'),
    // System PATH fallback
    'rustalk'
  ];

  for (const path of possiblePaths) {
    if (path === 'rustalk' || existsSync(path)) {
      return path;
    }
  }

  console.error('❌ rustalk binary not found. Please run: cargo build --release --workspace');
  process.exit(1);
}

const rustalkBinary = findRustalkBinary();

// If no arguments provided, show help (delegate to rustalk binary which shows full help)
if (process.argv.length === 2) {
  const child = spawn(rustalkBinary, [], {
    stdio: 'inherit',
    shell: currentPlatform === 'win32'
  });
  
  child.on('exit', (code) => {
    process.exit(code || 0);
  });
  
  child.on('error', (error) => {
    console.error('❌ Failed to execute rustalk:', error.message);
    process.exit(1);
  });
} else {
  // Forward all arguments to the Rust binary
  const child = spawn(rustalkBinary, process.argv.slice(2), {
    stdio: 'inherit',
    shell: currentPlatform === 'win32'
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });

  child.on('error', (error) => {
    console.error('❌ Failed to execute rustalk:', error.message);
    process.exit(1);
  });
}