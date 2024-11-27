#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

VALIDA_DIR="$HOME/.valida/"
VALIDA_BIN_DIR="$HOME/.local/bin/"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

mkdir -p "$VALIDA_DIR"
mkdir -p "$VALIDA_BIN_DIR"

cp -R "$SCRIPT_DIR"/* "$VALIDA_DIR"
install -t "$VALIDA_BIN_DIR" "$SCRIPT_DIR/valida-shell"
chmod +x "$VALIDA_BIN_DIR/valida-shell"

rustup toolchain link valida "$VALIDA_DIR"

# Store the correct profile file (i.e. .profile for bash or .zshenv for ZSH).
case $SHELL in
*/zsh)
    PROFILE=${ZDOTDIR-"$HOME"}/.zshenv
    PREF_SHELL=zsh
    ;;
*/bash)
    PROFILE=$HOME/.bashrc
    PREF_SHELL=bash
    ;;
*/fish)
    PROFILE=$HOME/.config/fish/config.fish
    PREF_SHELL=fish
    ;;
*/ash)
    PROFILE=$HOME/.profile
    PREF_SHELL=ash
    ;;
*)
    echo "valida: could not detect shell, manually add ${VALIDA_BIN_DIR} to your PATH."
    exit 1
esac

# Only add valida if it isn't already in PATH.
if [[ ":$PATH:" != *":${VALIDA_BIN_DIR}:"* ]]; then
    # Add the valida-shell directory to the path and ensure the old PATH variables remain.
    echo >> $PROFILE && echo "export PATH=\"\$PATH:$VALIDA_BIN_DIR\"" >> $PROFILE
fi

echo "Valida toolchain installed to $VALIDA_DIR, you should be able to run valida-shell"
