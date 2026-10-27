# Contributing

Use Rust 1.90 or later and nightly rustfmt. Run `./scripts/ci.sh` to check formatting,
generated data, Clippy, tests and documentation. `cargo machete` checks unused
dependencies, and `cargo deny check all` checks security and licenses.
Use `cargo +nightly fmt --all` to fix formatting.

Public APIs have doc comments and examples. The crate forbids unsafe code and
denies Clippy's pedantic lints. Public API tests live in `tests/`.

Browser data is checked in. See [tools/README.md](tools/README.md) for capturing
real browsers and regenerating `src/data.rs`. Keep captures, generated data and
tests together in a change. Update `CHANGELOG.md` for code and data changes.

The Browser data workflow proposes weekly capture updates. The Release workflow
publishes the version in `Cargo.toml` and creates a GitHub release; it requires
the `CRATES_TOKEN` repository secret. Before a release, bump the package version
and move changelog entries into a versioned section. No workflow runs during
Cargo builds.

Submit focused PRs against `main`, with a description of the resulting behavior
and relevant validation. Use Conventional Commit messages, such as
`feat: add browser profiles` or `fix: validate language preferences`.

Contributions are licensed under the repository's MIT license.
