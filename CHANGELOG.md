# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Optional argument for minimum file size for which a user would like to perform file size reduction.
  - It can come in three sizes: S, M, L, for 100 kB, 500 kB and 1 MB, respectively.

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
