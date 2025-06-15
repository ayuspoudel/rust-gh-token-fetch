#!/usr/bin/env bash

set -e

REPO="ayuspoudel/rust-token-fetch"
VERSION="${VERSION:-latest}"
INSTALL_DIR="/usr/local/bin"
TMP_DIR="$(mktemp -d)"

ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

echo "Detecting system: $OS-$ARCH"

# Map to expected binary directory
if [[ "$OS" == "darwin" ]]; then
  PLATFORM="macos"
elif [[ "$OS" == "linux" ]]; then
  PLATFORM="linux"
else
  echo "Unsupported OS: $OS"
  exit 1
fi

if [[ "$VERSION" == "latest" ]]; then
  VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -oP '"tag_name": "\K(.*)(?=")')
fi

echo "Downloading rust-token-fetch $VERSION for $PLATFORM..."

TARBALL="rust-token-fetch-${VERSION}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$TARBALL"

cd "$TMP_DIR"
curl -sL "$URL" -o "$TARBALL"
tar -xzf "$TARBALL"

echo "Installing to $INSTALL_DIR..."
sudo cp "rust-token-fetch-${VERSION}/${PLATFORM}/rust-token-fetch" "$INSTALL_DIR"
sudo chmod +x "$INSTALL_DIR/rust-token-fetch"

echo "rust-token-fetch installed to $INSTALL_DIR"
echo "Run 'rust-token-fetch --help' to get started"

cd -
rm -rf "$TMP_DIR"
