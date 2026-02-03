#!/bin/bash
set -e

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    darwin)
        case "$ARCH" in
            arm64) PLATFORM="aarch64-apple-darwin" ;;
            x86_64) PLATFORM="x86_64-apple-darwin" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    linux)
        case "$ARCH" in
            aarch64|arm64) PLATFORM="aarch64-unknown-linux-gnu" ;;
            x86_64) PLATFORM="x86_64-unknown-linux-gnu" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Get latest release URL
RELEASE_URL="https://github.com/jo-inc/skill-search/releases/latest/download/skill-search-${PLATFORM}.tar.gz"

# Install to ~/.local/bin (create if needed)
INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Downloading skill-search for ${PLATFORM}..."
curl -fsSL "$RELEASE_URL" | tar -xz -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/skill-search"

echo "Installed skill-search to $INSTALL_DIR/skill-search"

# Check if in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo ""
    echo "Add to your PATH: export PATH=\"\$HOME/.local/bin:\$PATH\""
fi
