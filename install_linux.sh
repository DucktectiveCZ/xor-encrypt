#!/bin/bash

TARGET_PATH="$HOME/.local/bin/xor-encrypt"
BINARY_PATH="./target/release/xor_encrypt"

echo "Building project..."
cargo build --release

echo "Copying binaries..."
cp "$BINARY_PATH" "$TARGET_PATH"

echo "Done! Please make sure that '$HOME/.local/bin/' is in your \$PATH."

