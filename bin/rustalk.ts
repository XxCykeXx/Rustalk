#!/usr/bin/env npx tsx

import { program } from 'commander';
import chalk from 'chalk';
import { execSync } from 'child_process';
import { existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Find rus binary
function findRusBinary(): string {
  const possible_paths = [
    // Local development
    join(__dirname, '..', 'target', 'release', process.platform === 'win32' ? 'rus.exe' : 'rus'),
    // NPM global install
    join(__dirname, process.platform === 'win32' ? 'rus.exe' : 'rus'),
    // System PATH
    'rus'
  ];
  
  for (const path of possible_paths) {
    if (path === 'rus' || existsSync(path)) {
      return path;
    }
  }
  
  return 'rus'; // fallback to PATH
}

const RUS_BINARY = findRusBinary();

program
  .name('rustalk')
  .description('Secure P2P terminal chat application - unified interface for reach + rus')
  .version('0.0.0');

// All chat functionality handled by rus
program
  .command('setup')
  .description('Setup new user credentials')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('🔧 Setting up user credentials...'));
    try {
      execSync(`"${RUS_BINARY}" setup`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to setup credentials.'));
      console.error(chalk.yellow('💡 Make sure Rustalk is properly installed with: npm install -g rustalk'));
      console.error(chalk.gray('   Or build from source with: cargo build --release --workspace'));
      process.exit(1);
    }
  });

program
  .command('start')
  .alias('chat')
  .description('Start the chat application')
  .option('-p, --port <number>', 'Port to listen on', '5000')
  .action(async (options: { port: string }): Promise<void> => {
    console.log(chalk.blue(`🚀 Starting Rustalk chat on port ${options.port}...`));
    try {
      execSync(`"${RUS_BINARY}" chat --port ${options.port}`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to start chat:'), error);
      process.exit(1);
    }
  });

program
  .command('connect')
  .description('Connect to a peer directly')
  .argument('<address>', 'Peer address (IP:PORT)')
  .option('-p, --port <number>', 'Port to listen on', '5000')
  .action(async (address: string, options: { port: string }): Promise<void> => {
    console.log(chalk.blue(`🔗 Connecting to ${address}...`));
    try {
      execSync(`"${RUS_BINARY}" connect ${address} --port ${options.port}`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to connect:'), error);
      process.exit(1);
    }
  });

program
  .command('info')
  .description('Show user information')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('ℹ️  Showing user information...'));
    try {
      execSync(`"${RUS_BINARY}" info`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to show info:'), error);
      process.exit(1);
    }
  });

program
  .command('send')
  .description('Send a quick message')
  .argument('<message>', 'Message to send')
  .option('-t, --to <peer>', 'Target peer ID')
  .action(async (message: string, options: { to?: string }): Promise<void> => {
    console.log(chalk.blue('📤 Sending message...'));
    try {
      const command = options.to 
        ? `"${RUS_BINARY}" send "${message}" --to "${options.to}"`
        : `"${RUS_BINARY}" send "${message}"`;
      execSync(command, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to send message:'), error);
      process.exit(1);
    }
  });

program
  .command('peers')
  .description('List connected peers')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('👥 Listing connected peers...'));
    try {
      execSync(`"${RUS_BINARY}" peers`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to list peers:'), error);
      process.exit(1);
    }
  });

program
  .command('nick')
  .description('Set display name')
  .argument('<name>', 'New display name')
  .action(async (name: string): Promise<void> => {
    console.log(chalk.blue(`👤 Setting display name to: ${name}`));
    try {
      execSync(`"${RUS_BINARY}" nick "${name}"`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to set display name:'), error);
      process.exit(1);
    }
  });

program
  .command('reset')
  .description('Reset configuration')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('🔄 Resetting configuration...'));
    try {
      execSync(`"${RUS_BINARY}" reset`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to reset:'), error);
      process.exit(1);
    }
  });

// PATH and user management - delegate to rus
program
  .command('add-path')
  .description('Add Rustalk binaries to system PATH')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('🔧 Adding rustalk and rus to system PATH...'));
    try {
      execSync(`"${RUS_BINARY}" add-path`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to add to PATH:'), error);
      process.exit(1);
    }
  });

program
  .command('remove-path')
  .description('Remove Rustalk binaries from system PATH')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('🔧 Removing rustalk and rus from system PATH...'));
    try {
      execSync(`"${RUS_BINARY}" remove-path`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to remove from PATH:'), error);
      process.exit(1);
    }
  });

program
  .command('list-users')
  .description('List all registered users')
  .action(async (): Promise<void> => {
    try {
      execSync(`"${RUS_BINARY}" list-users`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to list users:'), error);
      process.exit(1);
    }
  });

program
  .command('switch-user')
  .description('Switch to a different user')
  .argument('<email>', 'Email of user to switch to')
  .action(async (email: string): Promise<void> => {
    try {
      execSync(`"${RUS_BINARY}" switch-user "${email}"`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to switch user:'), error);
      process.exit(1);
    }
  });

// Default action
if (process.argv.length === 2) {
  console.log(chalk.cyan.bold('🚀 Welcome to Rustalk!'));
  console.log(chalk.gray('Modular P2P communication platform with end-to-end encryption\n'));
  
  console.log(chalk.blue.bold('🏗️  Architecture:'));
  console.log(chalk.gray('  • reach  - Core P2P networking library'));
  console.log(chalk.gray('  • rus    - Direct CLI interface'));
  console.log(chalk.gray('  • rustalk - Unified experience (you are here)\n'));
  
  console.log(chalk.yellow.bold('🚀 Quick Start:'));
  console.log(chalk.green('  rustalk setup              ') + '- Configure your identity');
  console.log(chalk.green('  rustalk chat               ') + '- Start P2P chat session');
  console.log(chalk.green('  rustalk connect <address>  ') + '- Connect to peer directly\n');
  
  console.log(chalk.yellow.bold('💬 Communication:'));
  console.log(chalk.green('  rustalk peers              ') + '- List connected peers');
  console.log(chalk.green('  rustalk send <peer> <msg>  ') + '- Send message to peer');
  console.log(chalk.green('  rustalk nick <name>        ') + '- Set display name');
  console.log(chalk.green('  rustalk info               ') + '- Show user information\n');
  
  console.log(chalk.yellow.bold('� User Management:'));
  console.log(chalk.green('  rustalk list-users         ') + '- List registered users');
  console.log(chalk.green('  rustalk switch-user <email>') + '- Switch active user\n');
  
  console.log(chalk.yellow.bold('🔧 System:'));
  console.log(chalk.green('  rustalk add-path           ') + '- Add to system PATH');
  console.log(chalk.green('  rustalk reset              ') + '- Reset configuration\n');
  
  console.log(chalk.blue('💡 ') + chalk.white('Use ') + chalk.cyan('"rus <command>"') + chalk.white(' for direct CLI access'));
  console.log(chalk.blue('📖 ') + chalk.white('For detailed help: ') + chalk.yellow('rustalk --help'));
  process.exit(0);
}

program.parse();