#!/usr/bin/env node
"use strict";

const { spawnSync } = require("child_process");
const path = require("path");
const os = require("os");
const fs = require("fs");

const PLATFORM = os.platform();
const ARCH = os.arch();

const PLATFORM_PACKAGES = {
  "win32-x64":   "fua-fua-win32-x64",
  "linux-x64":   "fua-fua-linux-x64",
  "darwin-x64":  "fua-fua-darwin-x64",
  "darwin-arm64": "fua-fua-darwin-arm64",
};

const BINARY_NAMES = {
  "win32-x64":   "fua-fua.exe",
  "linux-x64":   "fua-fua",
  "darwin-x64":  "fua-fua",
  "darwin-arm64": "fua-fua",
};

const platformKey = `${PLATFORM}-${ARCH}`;
const platformPkg = PLATFORM_PACKAGES[platformKey];
const binaryName  = BINARY_NAMES[platformKey];

if (!platformPkg) {
  console.error(
    `fua-fua: unsupported platform "${platformKey}". ` +
    `Supported: ${Object.keys(PLATFORM_PACKAGES).join(", ")}`
  );
  process.exit(1);
}

// Resolve binary from optional platform package.
let binaryPath;
try {
  const pkgDir = path.dirname(require.resolve(`${platformPkg}/package.json`));
  binaryPath = path.join(pkgDir, "bin", binaryName);
} catch {
  // Fall back to locally built binary (developer workflow).
  const localBinary = path.resolve(
    __dirname,
    "../../../target/release",
    PLATFORM === "win32" ? "fua-fua.exe" : "fua-fua"
  );
  if (fs.existsSync(localBinary)) {
    binaryPath = localBinary;
  } else {
    console.error(
      `fua-fua: could not locate binary.\n` +
      `Install the platform package: npm i ${platformPkg}\n` +
      `Or build locally: cargo build --release -p cli`
    );
    process.exit(1);
  }
}

const result = spawnSync(binaryPath, process.argv.slice(2), { stdio: "inherit" });
process.exit(result.status ?? 1);
