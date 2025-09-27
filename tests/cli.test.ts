import { describe, test, expect, beforeAll, afterAll } from 'bun:test';
import { execSync } from 'child_process';
import path from 'path';

describe('Rustalk CLI Tests', () => {
  let rustBinary: string;

  beforeAll(async () => {
    // Build the Rust binary for testing
    console.log('Building Rust binary for tests...');
    execSync('cargo build --release', { stdio: 'inherit' });
    
    rustBinary = process.platform === 'win32' 
      ? './target/release/rustalk.exe' 
      : './target/release/rustalk';
  });

  test('should show help when no arguments provided', () => {
    try {
      const output = execSync('bun run src/index.ts --help', { encoding: 'utf8' });
      expect(output).toContain('P2P Encrypted Chat Application');
      expect(output).toContain('Commands:');
    } catch (error: any) {
      // Commander.js exits with help text, check the stderr or stdout
      const errorOutput = error.stdout || error.stderr || '';
      expect(errorOutput).toContain('P2P Encrypted Chat Application');
      expect(errorOutput).toContain('Commands:');
    }
  });

  test('should show version', () => {
    const output = execSync('bun run src/index.ts --version', { encoding: 'utf8' });
    expect(output).toContain('0.1.0');
  });

  test('should build Rust components', () => {
    const output = execSync('bun run src/index.ts build', { encoding: 'utf8' });
    expect(output).toContain('Build completed successfully');
  });

  test('should show status command exists', () => {
    const output = execSync('bun run src/index.ts --help', { encoding: 'utf8' });
    expect(output).toContain('status');
    expect(output).toContain('Check online status of peers');
  });

  test('should show connect command exists', () => {
    const output = execSync('bun run src/index.ts --help', { encoding: 'utf8' });
    expect(output).toContain('connect');
    expect(output).toContain('Connect to a peer');
  });
});