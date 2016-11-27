# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Fixed

- The heap size is `end_addr` - `start_addr`. Previously, it was wrongly
  calculated as `end_addr - start_addr - 1`.

## [v0.2.0] - 2016-11-19

### Changed

- [breaking] Hid the HEAP variable. We now only expose an `init` function to
  initialize the allocator.

## v0.1.0 - 2016-11-19

### Added

- Initial version of the allocator

[Unreleased]: https://github.com/japaric/f3/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/f3/compare/v0.1.0...v0.2.0
