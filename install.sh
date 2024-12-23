#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Helper function to print an error message and exit.
error_exit() {
    echo "Error: $1 failed with exit code $?. Exiting."
    exit 1
}

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y || error_exit "Installing Rust"
source $HOME/.cargo/env

# Install the nightly toolchain
rustup toolchain install nightly --profile minimal -c rust-src || error_exit "Installing nightly toolchain"

# Set the default toolchain to nightly
rustup default nightly || error_exit "Setting default toolchain to nightly"

# Install the Succinct toolchain
curl -L https://sp1.succinct.xyz | bash || error_exit "Installing Succinct toolchain"
export PATH="$PATH:$HOME/.sp1/bin"
sp1up || error_exit "Updating Succinct toolchain"
cargo prove --version || error_exit "Checking cargo prove version"

# Install the jolt toolchain
rustup toolchain install nightly-2024-09-30
cargo +nightly-2024-09-30 install --git https://github.com/a16z/jolt --force --bins jolt || error_exit "Installing jolt toolchain"
# jolt install-toolchain || error_exit "Installing jolt runtime"

# Install the Risc0 toolchain
curl -L https://risczero.com/install | bash || error_exit "Installing Risc Zero toolchain"
export PATH="$PATH:$HOME/.risc0/bin"
rzup install || error_exit "Updating Risc Zero toolchain"
cargo risczero --version || error_exit "Checking cargo risczero version"

# Install Lita toolchain
wget https://github.com/lita-xyz/llvm-valida-releases/releases/download/v0.6.0-alpha/llvm-valida-v0.6.0-alpha-linux-x86_64.tar.xz || error_exit "Downloading lita toolchain"
tar xf llvm-valida-v0.6.0-alpha-linux-x86_64.tar.xz || error_exit "Extracting lita toolchain"
cp install_lita_toolchain.sh valida-toolchain/install.sh
cd valida-toolchain
./install.sh || error_exit "Installing lita toolchain"
cd ../
rm llvm-valida-v0.6.0-alpha-linux-x86_64.tar.xz
rm -rf valida-toolchain

# Install Nexus toolchain
cargo install --git https://github.com/nexus-xyz/nexus-zkvm cargo-nexus --tag 'v0.2.4' || error_exit "Installing nexus toolchain"
rustup target add riscv32i-unknown-none-elf || error_exit "Installing riscv32i architecture"
cargo nexus --help || error_exit "Checking cargo nexus version"

echo "All installations completed successfully."
