# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Implemented [`Allocator`] for `Heap` with the `allocator_api` crate feature.
  This feature requires a nightly toolchain for the unstable [`allocator_api`]
  compiler feature.

[`Allocator`]: https://doc.rust-lang.org/core/alloc/trait.Allocator.html
[`allocator_api`]: https://doc.rust-lang.org/beta/unstable-book/library-features/allocator-api.html

### Changed

- Updated `linked_list_allocator` dependency to 0.10.5, which allows
  compiling with stable rust.

## [v0.5.0] - 2022-12-06

### Changed

- Renamed crate from `alloc-cortex-m` to `embedded-alloc`.
- Renamed `CortexMHeap` to `Heap`.
- Use `critical-section` to lock the heap, instead of `cortex_m::interrupt::free()`.
  This allows using this crate on non-Cortex-M systems, or on
  Cortex-M systems that require a custom critical section implementation.

## [v0.4.3] - 2022-11-03

### Changed

- Updated `linked_list_allocator` dependency to 0.10.4, which fixes
  CVE-2022-36086/RUSTSEC-2022-0063.

## [v0.4.2] - 2022-01-04

### Changed

- Updated `cortex-m` dependency to 0.7.2.

## [v0.4.1] - 2021-01-02

### Added

- `const_mut_refs` feature to the dependency `linked_list_allocator` crate.

### Changed

- Bumped the dependency of the `linked_list_allocator` crate to v0.8.11.

## [v0.4.0] - 2020-06-05

- Bumped the `cortex-m` dependency to v0.6.2.
- Bumped the dependency of the `linked_list_allocator` crate to v0.8.1.
- Removed `#![feature(alloc)]` to supress compiler warning about stability for alloc.

## [v0.3.5] - 2018-06-19

### Fixed

- To work with recent nightly

## [v0.3.4] - 2018-05-12

### Changed

- Update the example in the crate level documentation to show how to define the new `oom` lang item.

## [v0.3.3] - 2018-04-23

- Bumped the dependency of the `linked_list_allocator` crate to v0.6.0.

## [v0.3.2] - 2018-02-26

### Changed

- Bumped the dependency of the `linked_list_allocator` crate to v0.5.0.

## [v0.3.1] - 2017-10-07

### Fixed

- The example in the documentation.

## [v0.3.0] - 2017-10-07

### Changed

- [breaking-change] Switched to the new allocator system. See documentation for details.

## [v0.2.2] - 2017-04-29

### Added

- a `__rust_allocate_zeroed` symbol as it's needed on recent nightlies.

## [v0.2.1] - 2016-11-27

### Fixed

- The heap size is `end_addr` - `start_addr`. Previously, it was wrongly
  calculated as `end_addr - start_addr - 1`.

## [v0.2.0] - 2016-11-19

### Changed

- [breaking-change] Hid the HEAP variable. We now only expose an `init` function to
  initialize the allocator.

## v0.1.0 - 2016-11-19

### Added

- Initial version of the allocator

[Unreleased]: https://github.com/rust-embedded/embedded-alloc/compare/v0.5.0...HEAD
[v0.5.0]: https://github.com/rust-embedded/embedded-alloc/compare/v0.4.3...v0.5.0
[v0.4.3]: https://github.com/rust-embedded/embedded-alloc/compare/v0.4.2...v0.4.3
[v0.4.2]: https://github.com/rust-embedded/embedded-alloc/compare/v0.4.1...v0.4.2
[v0.4.1]: https://github.com/rust-embedded/embedded-alloc/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/rust-embedded/embedded-alloc/compare/v0.3.5...v0.4.0
[v0.3.5]: https://github.com/rust-embedded/embedded-alloc/compare/v0.3.4...v0.3.5
[v0.3.4]: https://github.com/rust-embedded/embedded-alloc/compare/v0.3.3...v0.3.4
[v0.3.3]: https://github.com/rust-embedded/embedded-alloc/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/rust-embedded/embedded-alloc/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/rust-embedded/embedded-alloc/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/rust-embedded/embedded-alloc/compare/v0.2.2...v0.3.0
[v0.2.2]: https://github.com/rust-embedded/embedded-alloc/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-embedded/embedded-alloc/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded/embedded-alloc/compare/v0.1.0...v0.2.0
