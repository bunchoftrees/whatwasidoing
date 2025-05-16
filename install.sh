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
# Add alias to ~/.zshrc if not present
if ! grep -q "alias wtfisthis=" ~/.zshrc 2>/dev/null; then
  echo 'alias wtfisthis="whatwasidoing"' >> ~/.zshrc
  echo '✅ Added alias: wtfisthis -> whatwasidoing (you may need to restart your shell)'
else
  echo 'ℹ️ Alias "wtfisthis" already exists in ~/.zshrc'
fi