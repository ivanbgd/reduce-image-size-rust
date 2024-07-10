# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Some form of concurrent execution, but this might not be necessary as some
  dependencies already use `rayon` and `crossbeam`.

## [0.2.3] - 2024-07-10

### Changed

- Updated several dependencies to newer versions.

## [0.2.2] - 2024-07-10

### Changed

- Updated `release.yml` to include support for macOS on x86-64 and Linux on ARM.
- Updated `README.md` to include support for macOS on x86-64 and Linux on ARM.

## [0.2.1] - 2024-06-14

### Changed

- Updated `README.md` with a note about support for Apple silicon, the M-series.
- Updated `README.md` with a note about `nasm` being required on Windows, but not on macOS with Apple silicon.

## [0.2.0] - 2024-01-30

### Added

- Optional argument for minimum file size for which a user would like to perform file size reduction.
    - It comes in three sizes: S, M, L, for 100 kB, 500 kB and 1 MB, respectively.
- Add some info messages: at startup, then for copying and for skipping files.
- Add a closing message that warns users in case of an error.
- GitHub action "ci.yml" ("release.yml" had already been there).

### Changed

- When source and destination folders are different, non-supported files will simply be copied to the destination.
    - Previously, they would be left out.
- Updated `README.md` with Examples and some new notes.

## [0.1.0] - 2023-12-29

This is the very first (initial) fully-functioning version of the library and the program.

### Added

- Library crate:
    - The main business logic function (public),
    - Helper functions (private).
- Binary (executable) crate, which uses the library.
- **JPEG** support.
- **PNG** support.
- "README.md".
- "LICENSE" ("MIT").
- "CHANGELOG.md".
