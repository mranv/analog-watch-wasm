#!/bin/bash
set -e

# Build Rust to WebAssembly
wasm-pack build --target web

# Install npm dependencies
cd www
npm install

# Build the React app
npm run build