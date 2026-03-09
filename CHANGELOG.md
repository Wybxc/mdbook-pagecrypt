# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/Wybxc/mdbook-pagecrypt/compare/v0.1.2...v0.2.0) - 2026-03-09

### Added

- Require `password` in `[output.pagecrypt]` configuration ([#4](https://github.com/Wybxc/mdbook-pagecrypt/pull/4))
- Encrypt `toc.html` when present ([#7](https://github.com/Wybxc/mdbook-pagecrypt/pull/7))

### Changed

- Make `password` required in documentation and examples

### Fixed

- Validate missing password in `PageCryptBuilder::build` ([#6](https://github.com/Wybxc/mdbook-pagecrypt/pull/6))

## [0.1.2](https://github.com/Wybxc/mdbook-pagecrypt/compare/v0.1.1...v0.1.2) - 2026-02-23

### Changed

- Upgrade to mdBook 0.5 API (`mdbook-html` and `mdbook-renderer`)
- Update Rust dependencies and build-time minification configuration
- Improve GitHub Actions workflows for pull request builds and release automation

### Fixed

- Remove unsupported `multilingual` option from docs configuration

## [0.1.1](https://github.com/Wybxc/mdbook-pagecrypt/compare/v0.1.0...v0.1.1) - 2025-02-15

### Added

- Set up GitHub Pages publishing workflow
- Add Release-plz automation workflow

### Changed

- Add package metadata (`description`, `categories`, `keywords`)
- Add documentation link in README
- Enable Rust cache in CI and update dependencies
