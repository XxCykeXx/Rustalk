#!/usr/bin/env node

import { program } from 'commander';
import chalk from 'chalk';
import { execSync } from 'child_process';
import { existsSync } from 'fs';
import path from 'path';

// Check if Rust binary exists
const rustBinary = process.platform === 'win32' 
  ? path.join(process.cwd(), 'target', 'release', 'rustalk_cli.exe')
  : path.join(process.cwd(), 'target', 'release', 'rustalk_cli');

program
  .name('rustalk')
  .description('Secure P2P terminal chat application')
  .version('0.0.0');

program
  .command('setup')
  .description('Setup new user credentials')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('🔧 Setting up user credentials...'));
    try {
      execSync(`"${rustBinary}" setup`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to setup credentials:'), error);
      process.exit(1);
    }
  });

program
  .command('start')
  .alias('chat')
  .description('Start the chat application')
  .option('-p, --port <number>', 'Port to listen on', '8080')
  .action(async (options: { port: string }): Promise<void> => {
    console.log(chalk.blue(`🚀 Starting Rustalk on port ${options.port}...`));
    try {
      execSync(`"${rustBinary}" start --port ${options.port}`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to start chat:'), error);
      process.exit(1);
    }
  });

program
  .command('info')
  .description('Show user information')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('ℹ️  Showing user information...'));
    try {
      execSync(`"${rustBinary}" info`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to show info:'), error);
      process.exit(1);
    }
  });

program
  .command('reset')
  .description('Reset configuration')
  .action(async (): Promise<void> => {
    console.log(chalk.blue('🔄 Resetting configuration...'));
    try {
      execSync(`"${rustBinary}" reset`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to reset:'), error);
      process.exit(1);
    }
  });

// Add the new commands for PATH management and user management
program
  .command('add-path')
  .description('Add Rustalk binary to system PATH')
  .action(async (): Promise<void> => {
    const { execSync } = await import('child_process');
    try {
      execSync('rustalk_cli add-path', { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('Failed to add to PATH:', error));
      process.exit(1);
    }
  });

program
  .command('remove-path')
  .description('Remove Rustalk binary from system PATH')
  .action(async (): Promise<void> => {
    const { execSync } = await import('child_process');
    try {
      execSync('rustalk_cli remove-path', { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('Failed to remove from PATH:', error));
      process.exit(1);
    }
  });

program
  .command('check-path')
  .description('Check if Rustalk binary is in system PATH')
  .action(async (): Promise<void> => {
    const { execSync } = await import('child_process');
    try {
      execSync('rustalk_cli check-path', { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('Failed to check PATH:', error));
      process.exit(1);
    }
  });

program
  .command('list-users')
  .description('List all registered users')
  .action(async (): Promise<void> => {
    const { execSync } = await import('child_process');
    try {
      execSync('rustalk_cli list-users', { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('Failed to list users:', error));
      process.exit(1);
    }
  });

program
  .command('switch-user')
  .description('Switch to a different user')
  .argument('<username>', 'Username to switch to')
  .action(async (username: string): Promise<void> => {
    const { execSync } = await import('child_process');
    try {
      execSync(`rustalk_cli switch-user "${username}"`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('Failed to switch user:', error));
      process.exit(1);
    }
  });

program
  .command('remove-user')
  .description('Remove a user from the registry')
  .argument('<username>', 'Username to remove')
  .action(async (username: string): Promise<void> => {
    const { execSync } = await import('child_process');
    try {
      execSync(`rustalk_cli remove-user "${username}"`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('Failed to remove user:', error));
      process.exit(1);
    }
  });

// Default action
if (process.argv.length === 2) {
  console.log(chalk.cyan.bold('🚀 Welcome to Rustalk (JavaScript Edition)!'));
  console.log('Please run one of the following commands:');
  console.log(chalk.green('  rustalk setup      ') + '- Configure your credentials');
  console.log(chalk.green('  rustalk chat       ') + '- Start chatting');
  console.log(chalk.green('  rustalk info       ') + '- Show user information');
  console.log(chalk.green('  rustalk reset      ') + '- Reset configuration');
  console.log(chalk.green('  rustalk add-path   ') + '- Add to system PATH');
  console.log(chalk.green('  rustalk remove-path') + '- Remove from PATH');
  console.log(chalk.green('  rustalk check-path ') + '- Check PATH status');
  console.log(chalk.green('  rustalk list-users ') + '- List all users');
  console.log(chalk.green('  rustalk switch-user') + '- Switch active user');
  console.log(chalk.green('  rustalk remove-user') + '- Remove user account');
  console.log('');
  console.log('For more help, run: ' + chalk.yellow('rustalk --help'));
  process.exit(0);
}

program.parse();