#!/usr/bin/env sh
set -e

if ! command -v cargo >/dev/null 2>&1; then
    echo "rust not found, installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
fi

cargo build --release
cp ./target/release/magicmap ./
echo "it's done. built ./magicmap"
