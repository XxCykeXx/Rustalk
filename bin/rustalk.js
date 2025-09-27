#!/usr/bin/env node

import { RustalkJS } from '../src/index.js';
import { program } from 'commander';
import chalk from 'chalk';

program
  .name('rustalk')
  .description('Secure P2P terminal chat application (JavaScript version)')
  .version('0.1.0');

program
  .command('setup')
  .description('Setup new user credentials')
  .action(async () => {
    const rustalk = new RustalkJS();
    await rustalk.setup();
  });

program
  .command('chat')
  .description('Start the chat application')
  .option('-p, --port <number>', 'Port to listen on', '5000')
  .action(async (options) => {
    const rustalk = new RustalkJS();
    await rustalk.startChat(parseInt(options.port));
  });

program
  .command('info')
  .description('Show user information')
  .action(async () => {
    const rustalk = new RustalkJS();
    await rustalk.showInfo();
  });

program
  .command('reset')
  .description('Reset configuration')
  .action(async () => {
    const rustalk = new RustalkJS();
    await rustalk.reset();
  });

// Default action
if (process.argv.length === 2) {
  console.log(chalk.cyan.bold('🚀 Welcome to Rustalk (JavaScript Edition)!'));
  console.log('Please run one of the following commands:');
  console.log(chalk.green('  rustalk setup  ') + '- Configure your credentials');
  console.log(chalk.green('  rustalk chat   ') + '- Start chatting');
  console.log(chalk.green('  rustalk info   ') + '- Show your information');
  console.log(chalk.green('  rustalk --help ') + '- Show detailed help');
}

program.parse();