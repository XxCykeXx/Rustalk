#!/usr/bin/env node

import { spawn } from 'child_process';
import chalk from 'chalk';

console.log(chalk.blue.bold(`
   
     
    
P2P Encrypted Chat - npm wrapper
`));

// Initialize and run Rust operations via rus CLI
function runRustalkOperations() {
    const args = process.argv.slice(2);
    
    // If no arguments provided, show help
    if (args.length === 0) {
        console.log(chalk.yellow(' Initializing Rustalk operations...'));
        console.log(chalk.cyan('   This npm package delegates to the rus CLI'));
        console.log();
        console.log(chalk.green('Usage:'));
        console.log(chalk.white('  npm start setup          # Setup user credentials'));
        console.log(chalk.white('  npm start chat           # Start chat session'));
        console.log(chalk.white('  npm start users list     # List all users'));
        console.log(chalk.white('  npm start path add       # Add to system PATH'));
        console.log();
        console.log(chalk.blue(' All operations are handled by the rus CLI'));
        return;
    }
    
    // Spawn rus CLI with provided arguments
    console.log(chalk.yellow(` Running: rus ${args.join(' ')}`));
    
    const child = spawn('rus', args, {
        stdio: 'inherit',
        shell: true
    });
    
    child.on('error', (error) => {
        console.error(chalk.red(' Failed to start rus CLI:'), error.message);
        console.error(chalk.yellow(' Make sure rus is installed and available in PATH'));
        process.exit(1);
    });
    
    child.on('close', (code) => {
        process.exit(code || 0);
    });
}

// Execute the Rust operations
runRustalkOperations();
