#!/bin/bash

set -e

echo "Building release binary..."
cargo build --release

BINARY=whatwasidoing
TARGET=./target/release/$BINARY
DEST=/usr/local/bin/$BINARY

if [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Detected macOS. Installing to /usr/local/bin..."

  sudo cp "$TARGET" "$DEST"
  echo "✅ Installed $BINARY to $DEST"

  if [[ ":$PATH:" != *":/usr/local/bin:"* ]]; then
    echo "⚠️ /usr/local/bin not in PATH. You may want to add this to your ~/.zshrc:"
    echo 'export PATH="/usr/local/bin:$PATH"'
  fi
else
  echo "Unsupported OS: $OSTYPE"
  exit 1
fi