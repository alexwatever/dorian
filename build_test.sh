#!/bin/bash
set -e

echo "🔍 Checking project structure..."
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found. Run this from the dorian directory."
    exit 1
fi

echo "📦 Running cargo check..."
cargo check

echo "🏗️ Building the project..."
cargo build

echo "✅ Build successful! You can now run the game with: cargo run" 