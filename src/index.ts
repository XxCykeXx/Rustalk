#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import inquirer from 'inquirer';
import { execSync } from 'child_process';
import { existsSync } from 'fs';
import path from 'path';

const program = new Command();

// Check if Rust binary exists
const rustBinary = process.platform === 'win32' 
  ? path.join(process.cwd(), 'target', 'release', 'rustalk_cli.exe')
  : path.join(process.cwd(), 'target', 'release', 'rustalk_cli');

const hasRustBinary = existsSync(rustBinary);

console.log(chalk.blue.bold(`
╦═╗┬ ┬┌─┐┌┬┐┌─┐┬  ┬┌─
╠╦╝│ │└─┐ │ ├─┤│  ├┴┐
╩╚═└─┘└─┘ ┴ ┴ ┴┴─┘┴ ┴
P2P Encrypted Chat Application
`));

program
  .name('rustalk')
  .description('Secure P2P terminal chat application')
  .version('0.1.0');

program
  .command('start')
  .description('Start Rustalk chat application')
  .option('-p, --port <port>', 'Port to listen on', '8080')
  .option('-e, --email <email>', 'Your email address')
  .option('--password <password>', 'Your password')
  .action(async (options) => {
    if (!hasRustBinary) {
      console.log(chalk.red('❌ Rust binary not found. Building...'));
      try {
        execSync('cargo build --release', { stdio: 'inherit' });
      } catch (error) {
        console.error(chalk.red('❌ Failed to build Rust binary'));
        process.exit(1);
      }
    }

    let email = options.email;
    let password = options.password;

    if (!email || !password) {
      const answers = await inquirer.prompt([
        {
          type: 'input',
          name: 'email',
          message: 'Enter your email address:',
          when: !email,
        },
        {
          type: 'password',
          name: 'password',
          message: 'Enter your password:',
          when: !password,
        }
      ]);

      email = email || answers.email;
      password = password || answers.password;
    }

    console.log(chalk.green(`🚀 Starting Rustalk with email: ${email}`));
    
    try {
      const cmd = `${rustBinary} --email "${email}" --password "${password}" --port ${options.port}`;
      execSync(cmd, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Error starting Rustalk:'), error);
    }
  });

program
  .command('connect')
  .description('Connect to a peer')
  .argument('<address>', 'Peer address (IP:PORT)')
  .action((address) => {
    if (!hasRustBinary) {
      console.error(chalk.red('❌ Rust binary not found. Run "rustalk start" first.'));
      process.exit(1);
    }
    
    console.log(chalk.blue(`🔗 Connecting to peer: ${address}`));
    try {
      execSync(`${rustBinary} connect ${address}`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Connection failed:'), error);
    }
  });

program
  .command('status')
  .description('Check online status of peers')
  .action(() => {
    if (!hasRustBinary) {
      console.error(chalk.red('❌ Rust binary not found. Run "rustalk start" first.'));
      process.exit(1);
    }
    
    try {
      execSync(`"${rustBinary}" status`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Status check failed:'), error);
    }
  });

program
  .command('build')
  .description('Build the Rust components')
  .action(() => {
    console.log(chalk.blue('🔨 Building Rust components...'));
    try {
      execSync('cargo build --release', { stdio: 'inherit' });
      console.log(chalk.green('✅ Build completed successfully!'));
    } catch (error) {
      console.error(chalk.red('❌ Build failed:'), error);
      process.exit(1);
    }
  });

program
  .command('setup')
  .description('Setup initial user configuration')
  .action(() => {
    console.log(chalk.blue('🔧 Setting up Rustalk configuration...'));
    try {
      execSync(`"${rustBinary}" setup`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Setup failed:'), error);
    }
  });

program
  .command('test')
  .description('Run tests')
  .action(() => {
    console.log(chalk.blue('🧪 Running tests...'));
    try {
      execSync('cargo test', { stdio: 'inherit' });
      execSync('bun test', { stdio: 'inherit' });
      console.log(chalk.green('✅ All tests passed!'));
    } catch (error) {
      console.error(chalk.red('❌ Tests failed:'), error);
      process.exit(1);
    }
  });

program
  .command('add-path')
  .description('Add Rustalk to system PATH')
  .action(() => {
    console.log(chalk.blue('➕ Adding Rustalk to system PATH...'));
    try {
      execSync(`"${rustBinary}" add-path`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to add to PATH:'), error);
    }
  });

program
  .command('remove-path')
  .description('Remove Rustalk from system PATH')
  .action(() => {
    console.log(chalk.blue('➖ Removing Rustalk from system PATH...'));
    try {
      execSync(`"${rustBinary}" remove-path`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to remove from PATH:'), error);
    }
  });

program
  .command('check-path')
  .description('Check if Rustalk is in system PATH')
  .action(() => {
    try {
      execSync(`"${rustBinary}" check-path`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Path check failed:'), error);
    }
  });

program
  .command('list-users')
  .alias('users')
  .description('List all registered users')
  .action(() => {
    try {
      execSync(`"${rustBinary}" list-users`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to list users:'), error);
    }
  });

program
  .command('switch-user')
  .description('Switch to a different user')
  .argument('<user-id>', 'User ID to switch to')
  .action((userId) => {
    console.log(chalk.blue(`🔄 Switching to user: ${userId}`));
    try {
      execSync(`"${rustBinary}" switch-user ${userId}`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to switch user:'), error);
    }
  });

program
  .command('remove-user')
  .description('Remove a user from the registry')
  .argument('<user-id>', 'User ID to remove')
  .action((userId) => {
    console.log(chalk.blue(`🗑️  Removing user: ${userId}`));
    try {
      execSync(`"${rustBinary}" remove-user ${userId}`, { stdio: 'inherit' });
    } catch (error) {
      console.error(chalk.red('❌ Failed to remove user:'), error);
    }
  });

// Parse command line arguments
program.parse();

// If no command provided, show help
if (!process.argv.slice(2).length) {
  program.outputHelp();
}