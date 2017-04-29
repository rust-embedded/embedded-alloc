# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.2] - 2017-04-29

### Added

- a `__rust_allocate_zeroed` symbol as it's needed on recent nightlies.

## [v0.2.1] - 2016-11-27

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

[Unreleased]: https://github.com/japaric/alloc-cortex-m/compare/v0.2.2...HEAD
[v0.2.2]: https://github.com/japaric/alloc-cortex-m/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/japaric/alloc-cortex-m/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/alloc-cortex-m/compare/v0.1.0...v0.2.0
