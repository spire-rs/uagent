#!/bin/sh
# Run formatting, generated-data, Clippy, test and documentation checks.
# Usage: ./scripts/ci.sh
# Requires Rust 1.90+, nightly rustfmt, and Python 3.
# Can be called from any working directory; exits on the first failed command.
set -eu

# Run from the repository root, regardless of the caller's working directory.
CDPATH= cd "$(dirname "$0")/.."

# Check all feature configurations.
cargo check --no-default-features
cargo check --all-features
# Check Rust formatting and generated browser data without modifying them.
cargo +nightly fmt --all -- --check
python3 tools/update.py --check
# Deny compiler and Clippy warnings across library and test targets.
cargo clippy --all-targets --all-features -- -D warnings
# Run the public API, capture and documentation tests.
cargo test --all-features
# Build documentation with the same flags as the Docs workflow.
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --no-deps --all-features

printf 'All CI checks passed.\n'
