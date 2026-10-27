#!/bin/sh
set -eu

# Run from the repository root, regardless of the caller's working directory.
CDPATH= cd "$(dirname "$0")/.."

cargo check --no-default-features
cargo check --all-features
cargo +nightly fmt --all -- --check
python3 tools/update.py --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --no-deps --all-features

printf 'All CI checks passed.\n'
