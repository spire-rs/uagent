# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Browser`, `Platform` and `Profile` for Chrome, Edge, Firefox and Safari
  user agents and navigation headers, including Chromium client hints.
- `UserAgents` for deterministic profile selection by relative browser and
  platform weights, with configurable versions and language preferences.
- `Header` for named header fields in captured data and generated profiles.
- BCP 47 language-tag and Accept-Language quality-value validation with oxilangtag.
- Real browser captures, a shared capture server, and offline data generation.
- Build, security, Dependabot, browser-data and release workflows, with
  `scripts/ci.sh` for local checks.
