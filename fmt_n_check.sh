#!/usr/bin/env bash

# This script formats and checks the codebase for errors.
# It is intended to be run before committing changes to the repository.

set -e

# Format the codebase.
echo "Formatting the codebase..."
cargo fmt --all -- --check

# Check the codebase for errors.
echo "Checking the codebase for errors..."
cargo check --all-features

# Check wasm target
echo "Checking wasm target..."
cargo check --all-features --lib --target wasm32-unknown-unknown
