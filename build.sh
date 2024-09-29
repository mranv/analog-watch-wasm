#!/bin/bash
set -e

echo "Building Rust to WebAssembly..."
wasm-pack build --target web

echo "Copying WebAssembly files to www/pkg directory..."
mkdir -p www/pkg
cp pkg/* www/pkg/

echo "Installing npm dependencies..."
cd www
npm install

echo "Building the React app..."
npm run build

echo "Build completed successfully!"
echo "Contents of www/dist directory:"
ls -R ../www/dist

echo "Contents of www/pkg directory:"
ls -R ../www/pkg