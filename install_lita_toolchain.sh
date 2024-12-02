#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

VALIDA_DIR="/valida-toolchain"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
VALIDA_BIN_DIR="$HOME/.local/bin/"

sudo mkdir -p "$VALIDA_DIR"
mkdir -p "$VALIDA_BIN_DIR"

sudo cp -R "$SCRIPT_DIR"/* "$VALIDA_DIR"
install -t "$VALIDA_BIN_DIR" "$SCRIPT_DIR/valida-shell"
chmod +x "$VALIDA_BIN_DIR/valida-shell"

rustup toolchain link valida "$VALIDA_DIR"


echo "Valida toolchain installed to $VALIDA_DIR, you should be able to run valida-shell"
