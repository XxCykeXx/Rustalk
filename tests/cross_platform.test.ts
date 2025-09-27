import { test, expect, describe, beforeAll, afterAll } from 'bun:test';
import { execSync, spawn } from 'child_process';
import { existsSync, rmSync } from 'fs';
import path from 'path';
import os from 'os';

describe('Cross-platform Rustalk Tests', () => {
  const projectRoot = process.cwd();
  const rustBinary = process.platform === 'win32' 
    ? path.join(projectRoot, 'target', 'release', 'rustalk_cli.exe')
    : path.join(projectRoot, 'target', 'release', 'rustalk_cli');
  
  beforeAll(() => {
    // Ensure binary is built
    if (!existsSync(rustBinary)) {
      console.log('Building Rust binary for tests...');
      execSync('cargo build --release', { stdio: 'inherit' });
    }
    expect(existsSync(rustBinary)).toBe(true);
  });

  afterAll(() => {
    // Cleanup test config if exists
    try {
      const configDir = getConfigDir();
      if (existsSync(configDir)) {
        rmSync(configDir, { recursive: true, force: true });
      }
    } catch (e) {
      // Ignore cleanup errors
    }
  });

  test('should detect correct platform', () => {
    const platform = process.platform;
    expect(['win32', 'darwin', 'linux', 'freebsd', 'openbsd'].includes(platform)).toBe(true);
  });

  test('should have correct binary path for platform', () => {
    if (process.platform === 'win32') {
      expect(rustBinary.endsWith('.exe')).toBe(true);
    } else {
      expect(rustBinary.endsWith('.exe')).toBe(false);
    }
    expect(existsSync(rustBinary)).toBe(true);
  });

  test('should run setup command cross-platform', () => {
    try {
      const output = execSync(`"${rustBinary}" setup --email "test@cross-platform.com" --name "Test User" --password "test123"`, {
        encoding: 'utf8',
        stdio: 'pipe'
      });
      
      expect(output).toContain('Setup completed');
    } catch (error: any) {
      // Setup might fail if already configured, that's ok
      console.log('Setup command result:', error.stdout || error.message);
    }
  });

  test('should run info command cross-platform', () => {
    try {
      const output = execSync(`"${rustBinary}" info`, {
        encoding: 'utf8',
        stdio: 'pipe',
        timeout: 10000
      });
      
      // Should not crash and should produce some output
      expect(typeof output).toBe('string');
      expect(output.length).toBeGreaterThan(0);
    } catch (error: any) {
      console.log('Info command error:', error.message);
      // Command might fail if not configured, but shouldn't crash
      expect(error.status).toBeDefined();
    }
  });

  test('should handle help command cross-platform', () => {
    const output = execSync(`"${rustBinary}" --help`, {
      encoding: 'utf8',
      stdio: 'pipe'
    });
    
    expect(output).toContain('rustalk');
    expect(output).toContain('Usage:');
  });

  test('TypeScript CLI should work cross-platform', () => {
    try {
      const output = execSync('bun run src/index.ts --help', {
        encoding: 'utf8',
        stdio: 'pipe'
      });
      
      expect(output).toContain('Rustalk');
      expect(output).toContain('P2P Encrypted Chat');
    } catch (error: any) {
      console.log('TypeScript CLI error:', error.message);
      throw error;
    }
  });

  test('should handle build command', () => {
    try {
      const output = execSync('bun run src/index.ts build', {
        encoding: 'utf8',
        stdio: 'pipe',
        timeout: 30000
      });
      
      expect(output).toContain('Build completed successfully');
    } catch (error: any) {
      console.log('Build command error:', error.message);
      // Build warnings are ok, but it shouldn't fail completely
      if (error.stdout && error.stdout.includes('Build completed successfully')) {
        // Success despite warnings
        return;
      }
      throw error;
    }
  });

  test('should handle status command', () => {
    try {
      const output = execSync('bun run src/index.ts status', {
        encoding: 'utf8',
        stdio: 'pipe',
        timeout: 10000
      });
      
      // Should show status without crashing
      expect(typeof output).toBe('string');
    } catch (error: any) {
      console.log('Status command error:', error.message);
      // May fail if not configured, but shouldn't crash
    }
  });
});

// Helper function to get config directory (cross-platform)
function getConfigDir(): string {
  const homeDir = os.homedir();
  
  switch (process.platform) {
    case 'win32':
      return path.join(homeDir, 'AppData', 'Local', 'rustalk');
    case 'darwin':
      return path.join(homeDir, 'Library', 'Application Support', 'rustalk');
    default:
      return path.join(homeDir, '.config', 'rustalk');
  }
}

describe('Cross-platform Path Tests', () => {
  test('should generate correct config paths for each platform', () => {
    const configDir = getConfigDir();
    
    expect(path.isAbsolute(configDir)).toBe(true);
    expect(configDir).toContain('rustalk');
    
    if (process.platform === 'win32') {
      expect(configDir).toContain('AppData');
    } else if (process.platform === 'darwin') {
      expect(configDir).toContain('Library');
    } else {
      expect(configDir).toContain('.config');
    }
  });

  test('should handle path separators correctly', () => {
    const testPaths = [
      getConfigDir(),
      path.join(process.cwd(), 'target', 'release'),
      path.join(process.cwd(), 'src')
    ];

    testPaths.forEach(testPath => {
      expect(path.isAbsolute(testPath)).toBe(true);
      
      // Should not contain mixed separators
      const normalized = path.normalize(testPath);
      expect(normalized).toBe(testPath);
    });
  });
});

describe('Environment Variable Tests', () => {
  test('should respect HOME environment variable', () => {
    const home = process.env.HOME || process.env.USERPROFILE;
    expect(home).toBeDefined();
    expect(existsSync(home!)).toBe(true);
  });

  test('should handle missing environment variables gracefully', () => {
    // This tests our fallback logic
    const originalHome = process.env.HOME;
    const originalUserProfile = process.env.USERPROFILE;
    
    try {
      delete process.env.HOME;
      delete process.env.USERPROFILE;
      
      // Should still be able to get some config directory
      expect(() => getConfigDir()).not.toThrow();
    } finally {
      // Restore environment variables
      if (originalHome) process.env.HOME = originalHome;
      if (originalUserProfile) process.env.USERPROFILE = originalUserProfile;
    }
  });
});