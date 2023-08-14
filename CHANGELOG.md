# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.3] - 2023-08-14

### Added

- `cw-vault-standard-test-helpers` crate

### Fixed

- Remove faulty usage of `to_binary` in `VaultContract` query helper functions.

### Changed

- Changes type of `VaultStandardInfoResponse::version` from `u16` to `String`

## [0.3.2] - 2023-08-12

### Added

- Export `const VERSION`.

## [0.3.1] - 2023-07-19

### Added

- Adds helper module with helper structs `VaultContract` and `VaultContractUnchecked`.
