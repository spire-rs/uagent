# Contributing

Thank you for your interest in contributing to uagent! We appreciate your help in
making this crate better.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Code Style](#code-style)
- [Submitting Changes](#submitting-changes)
- [Releasing](#releasing)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

## Code of Conduct

This project follows the
[Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By
participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.90 or later, and a nightly toolchain for formatting
- Git
- Python 3 for generated-data checks and browser capture tools
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) and
  [cargo-machete](https://github.com/bnjbvr/cargo-machete), for the security
  and dependency checks

```bash
rustup toolchain install nightly --component rustfmt
cargo install cargo-deny cargo-machete --locked
```

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/uagent.git
   cd uagent
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/spire-rs/uagent.git
   ```

## Development Setup

The `scripts/ci.sh` script mirrors the CI checks:

```bash
# Check, format, clippy, tests and docs, as CI runs them
./scripts/ci.sh

# Fix the formatting with nightly rustfmt
cargo +nightly fmt --all

# Check advisories, licenses and sources with cargo-deny
cargo deny check all

# Find unused dependencies
cargo machete
```

## Making Changes

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following our [code style](#code-style)

3. Add tests for any new functionality

4. Ensure all checks pass:
   ```bash
   ./scripts/ci.sh
   ```

5. Update the documentation and the crate's `CHANGELOG.md`

## Testing

- Unit tests of private code live next to it, in a `#[cfg(test)] mod test`.
- Tests of the public API live in the crate's `tests/` directory.
- Doc comments of public APIs include examples, which run as doc tests.
- Browser profile tests compare every header against checked-in captures. Run
  them after changing profile code or generated data:
  ```bash
  cargo test --all-features
  python3 tools/update.py --check
  ```
- See [tools/README.md](tools/README.md) for capturing browsers and regenerating
  `src/generated.rs`. Keep captures, generated data and tests together in a change.
- Language preferences are untrusted input, so test invalid language tags,
  quality values and header injection, not only well-formed preferences.

### Minimum Supported Rust Version

The crate supports Rust 1.90 (`rust-version` in `Cargo.toml`), and CI checks
it. Avoid newer language features and standard library APIs, or raise the
minimum supported version on purpose in a minor release:

```bash
cargo +1.90.0 check --all-features
```

## Code Style

- Format with nightly rustfmt (`cargo +nightly fmt --all`), which applies the
  import grouping in `rustfmt.toml` and a maximum line length of 100 characters.
- `clippy::pedantic` is denied for the crate, so `./scripts/ci.sh` must pass
  with no warnings. CI runs clippy on the toolchain in
  `.github/workflows/build.yml`, which may report lints that newer versions do
  not.
- The crate has `#![forbid(unsafe_code)]`.
- Document all public APIs with doc comments, including a `# Errors` section
  for functions that return a `Result`.
- Comments describe the code as it is, not how it changed.
- Keep functions focused and reasonably sized, with meaningful names.

## Submitting Changes

1. Commit your changes with [Conventional Commits](https://www.conventionalcommits.org)
   messages:
   ```bash
   git commit -m "fix(uagent): reject invalid language preferences"
   git commit -m "feat(uagent): add browser profiles"
   git commit -m "chore(deps): update dependencies"
   ```

2. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a Pull Request on GitHub with:
   - A clear title describing the change
   - A detailed description of what changed and why
   - Reference to any related issues
   - Any breaking changes highlighted

### Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Update the crate's `CHANGELOG.md` with your changes
- Ensure CI passes, including the semver check of the public API
- Be responsive to review feedback

## Releasing

To release the crate:

1. Bump the `version` in `Cargo.toml`: the minor version for breaking
   changes while it is below 1.0, the patch version otherwise.
2. Move the `CHANGELOG.md` entries under the new version and date.
3. Merge the changes into `main`.
4. Create and push a `v<version>` tag on `main`, such as `v0.1.0`. The **Release**
   workflow creates the GitHub release and publishes the crate to crates.io.
   The repository needs a `CRATES_TOKEN` secret for publishing.

## Reporting Bugs

If you find a bug, please create an issue on GitHub with:

- A clear, descriptive title
- The crate and its version
- Steps to reproduce the issue, ideally with the browser profile or language input
- Expected behavior
- Actual behavior
- Rust version and operating system

### Bug Report Template

```markdown
**Describe the bug** A clear and concise description of the bug.

**To Reproduce** The input and the code that reproduce the behavior:

1. ...
2. ...

**Expected behavior** What you expected to happen.

**Actual behavior** What actually happened.

**Environment**

- Crate and version: [e.g. uagent 0.1.0]
- Rust version: [e.g. 1.90.0]
- OS: [e.g. Ubuntu 24.04]

**Additional context** Any other relevant information.
```

## Feature Requests

We welcome feature requests! Please create an issue with:

- A clear description of the feature
- Use cases and benefits, and the specification it follows, if any
- Any implementation ideas you have
- Whether you'd be willing to implement it

## Questions?

If you have questions about contributing, feel free to ask in the issue
tracker.

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.
