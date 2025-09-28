#!/usr/bin/env node

import { spawn } from 'child_process';
import { existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { platform } from 'os';

const __dirname = dirname(fileURLToPath(import.meta.url));
const currentPlatform = platform();

function findRusBinary(): string {
  const possiblePaths = [
    // From compiled dist (when installed via npm)
    join(__dirname, '..', '..', 'target', 'release', currentPlatform === 'win32' ? 'rus.exe' : 'rus'),
    // From bin directory (development)
    join(__dirname, '..', '..', 'bin', currentPlatform === 'win32' ? 'rus.exe' : 'rus'),
    // From target debug (development)
    join(__dirname, '..', '..', 'target', 'debug', currentPlatform === 'win32' ? 'rus.exe' : 'rus'),
    // System PATH fallback
    'rus'
  ];

  for (const path of possiblePaths) {
    if (path === 'rus' || existsSync(path)) {
      return path;
    }
  }

  console.error('❌ rus binary not found. Please run: cargo build --release --workspace');
  process.exit(1);
}

const rusBinary = findRusBinary();

// Forward all arguments to the Rust binary
const child = spawn(rusBinary, process.argv.slice(2), {
  stdio: 'inherit',
  shell: currentPlatform === 'win32'
});

child.on('exit', (code) => {
  process.exit(code || 0);
});

child.on('error', (error) => {
  console.error('❌ Failed to execute rus:', error.message);
  process.exit(1);
});