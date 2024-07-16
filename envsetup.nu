#!/usr/bin/env nu

rustup toolchain install stable
cargo update
cargo clean
echo "Setup build environment completed."