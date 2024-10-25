# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.1](https://github.com/petit-chat/petit-filou/compare/v2.0.0...v2.0.1) - 2024-10-25

### Other

- *(mime types)* [**breaking**] use rust features

## [2.0.0](https://github.com/petit-chat/petit-filou/compare/v1.1.1...v2.0.0) - 2024-10-18

### Added

- [**breaking**] support multiple mime types other than mp4 ([#12](https://github.com/petit-chat/petit-filou/pull/12))

## [1.1.1](https://github.com/petit-chat/petit-filou/compare/v1.1.0...v1.1.1) - 2024-10-18

### Fixed

- *(pf_lib)* verify that existing urls are mp4 videos

## [1.1.0](https://github.com/petit-chat/petit-filou/compare/v1.0.0...v1.1.0) - 2024-10-13

### Added

- Automated release management with release-plz and ci/cd workflow (#5)

## [1.0.0](https://github.com/petit-chat/petit-filou/compare/v0.3.1...v1.0.0) - 2024-10-10

### Added

- Unit and integration tests.

### Changed

- Split project in two different packages, pf_lib containing the business logic and pf_cli containing the user interface. This logic allows the publishing of pf_lib to crates.io.

### Removed

- CD workflow.
- Useless documentation.
- Stop using [git-cliff](https://github.com/orhun/git-cliff).
- CLI completions and man page.

## [0.3.1](https://github.com/petit-chat/petit-filou/compare/v0.3.0...v0.3.1) - 2024-08-20

### Changed

- Reduce nested iterator complexity in URL extractor functions.
- Split CI workflow steps into separate jobs in order to improve failure predictability.

### Fixed

- Typos and broken URLs in documentation.
- CD now building with checkout to the right ref (#3).

## [0.3.0](https://github.com/petit-chat/petit-filou/compare/v0.2.1...v0.3.0) - 2024-08-19

### Added

- Generation of man page and CLI completions.
- Fully automated CD building and uploading assets to release.
- Using [git-cliff](https://github.com/orhun/git-cliff).

### Changed

- Move CLI args to a dedicated module.
- CI now running for pull request towards main branch only.

### Removed

- Release workflow.

## [0.2.1](https://github.com/petit-chat/petit-filou/compare/v0.2.0...v0.2.1) - 2024-08-16

### Changed

- Dependencies bump.

## [0.2.0](https://github.com/petit-chat/petit-filou/compare/v0.1.0...v0.2.0) - 2024-08-16

### Added

- Release workflow uploading binary files to releases.
- Issue templates.

### Fixed

- Broken URLs in documentation.

## [0.1.0](https://github.com/petit-chat/petit-filou/releases/tag/v0.1.0) - 2024-08-16

### Added

- This project to help people retrieve video URLs on WordPress websites.
