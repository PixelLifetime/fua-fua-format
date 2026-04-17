#!/usr/bin/env node
const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');

// Point directly to your local Rust build output
const isWin = os.platform() === 'win32';
const binaryName = isWin ? 'fua-fua.exe' : 'fua-fua';
const binaryPath = path.resolve(__dirname, '../../../target/release/', binaryName);

// Pass all arguments from the user directly to the Rust CLI
const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit'
});

process.exit(result.status || 0);
