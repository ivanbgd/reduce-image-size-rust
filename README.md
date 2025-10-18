# Reduce Image Size

[![license](https://img.shields.io/badge/License-MIT-blue.svg?style=flat)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/reduce_image_size.svg)][crates.io link]
[![downloads](https://img.shields.io/crates/d/reduce_image_size.svg)][crates.io link]  
[![docs.rs](https://docs.rs/reduce_image_size/badge.svg)](https://docs.rs/reduce_image_size/)
[![CI](https://github.com/ivanbgd/reduce-image-size-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/ivanbgd/reduce-image-size-rust/actions/workflows/ci.yml)
[![Security audit](https://github.com/ivanbgd/reduce-image-size-rust/actions/workflows/audit.yml/badge.svg)](https://github.com/ivanbgd/reduce-image-size-rust/actions/workflows/audit.yml)  
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)

[crates.io link]: https://crates.io/crates/reduce_image_size

## Description
Reduces size of images in a folder (and optionally sub-folders, recursively).

This is useful for archiving of photos, for example, as they look the same on a display even with a reduced file size.  
This application reduces file sizes of images in bulk.

Supports JPEG and PNG image formats, with the following file extensions (case-insensitive): `jpg`, `jpeg`, `png`.

Supports Windows, macOS on Apple silicon (ARM) and x86-64, and Linux on ARM and x86-64.

Executable files for Windows, macOS and Linux can be downloaded from
the [Releases](https://github.com/ivanbgd/reduce-image-size-rust/releases) page of the repository.

By default, keeps the original images and creates copies with reduced file size.

By default, copies the entire folder tree, with all sub-folders that exist in the source tree.  
The target folder tree will be created automatically,
and the new reduced-size images will be copied properly to their respective paths.  
It is only required to provide the root target folder, and it will also be created if it doesn't exist.  
Non-supported files will simply be copied to the destination.

The destination folder can be the same as the source folder, in which case the original images will be **overwritten**,
and not retained.  
Other, non-supported files, will be retained.

If there is enough disk space, it is advised to specify a different destination folder than the source folder,
so that the original images can be retained and the newly-created reduced-size images can be inspected for quality.  
A user can experiment with the `resize` and the `quality` arguments.  
Also, the user can go only one level deep and not recursively, or simply experiment on a copy of an image folder.  
If satisfied with the result, original images can be deleted afterwards easily to save disk and/or cloud space.

## Options
- Look into subdirectories recursively (process the entire tree); recommended: `-r`, `--recursive`
- Reduce both image dimensions by half: `--resize`
- JPEG quality, on a scale from 1 (worst) to 100 (best); the default is 75; ignored in case of PNGs:
  `-q`, `--quality <QUALITY>`
- A minimum file size for which a user would like to perform file size reduction:
  `-s {s,m,l,S,M,L}`, `--size {s,m,l,S,M,L}`
    - S = 100 kB, M = 500 kB, L = 1 MB
    - Files that are smaller than the designated size will simply be copied to the destination folder.
    - If this option is left out, then all files are considered for size reduction; i.e., minimal considered size is 0.

### Examples
See below for how to prepare the application for running.  
The file paths in the examples are for Windows.
- `reduce_image_size D:\img_src D:\img_dst`
- `reduce_image_size D:\img_src D:\img_dst -r`
- `reduce_image_size D:\img_src D:\img_dst -r -s m`
- `reduce_image_size D:\img_src D:\img_dst --recursive --size L`
- `reduce_image_size D:\img_src D:\img_dst -r --resize -q 60 -s l`
- `reduce_image_size D:\img_src D:\img_dst --recursive --resize --quality 60 --size L`

## Notes
- Updated and tested in Rust 1.89.0 and 1.90.0 on Apple silicon with macOS Sequoia 15.3.2.
- First developed in Rust 1.74.1, but also tested later with Rust 1.79.0.
- Tested on x86-64 CPUs on Windows 10 and Windows 11.
- Tested on Apple silicon, M2 Pro, on macOS Sonoma 14.5.
- Also tested on WSL - Ubuntu 22.04.2 LTS (GNU/Linux 5.15.133.1-microsoft-standard-WSL2 x86_64) on Windows 11 @ x86-64.
- Linux wasn't tested directly, but should work, at least on x86-64 CPUs.

## Security

- [cargo audit](https://github.com/rustsec/rustsec/blob/main/cargo-audit/README.md) is supported,
  as well as its GitHub action, [audit-check](https://github.com/rustsec/audit-check).
- [cargo deny](https://embarkstudios.github.io/cargo-deny/) is supported,
  as well as its GitHub action, [cargo-deny-action](https://github.com/EmbarkStudios/cargo-deny-action).

## Development

### Pre-commit

[pre-commit](https://pre-commit.com/) hooks are supported.

```shell
$ pip install pre-commit  # If you don't already have pre-commit installed on your machine. Run once.
$ pre-commit autoupdate  # Update hook repositories to the latest versions.
$ pre-commit install  # Sets up the pre-commit git hook script for the repository. Run once.
$ pre-commit install --hook-type pre-push  # Sets up the pre-push git hook script for the repository. Run once.
$ pre-commit run  # For manual running; considers only modified files.
$ pre-commit run --all-files  # For manual running; considers all files.
```

After installing it, the provided [pre-commit hook(s)](.pre-commit-config.yaml) will run automatically on `git commit`.

## Running the Application
Executable files for Windows, macOS and Linux can be downloaded from
the [Releases](https://github.com/ivanbgd/reduce-image-size-rust/releases) page of the repository.

Use the latest release version.

Download the appropriate archive for your OS and unpack it to a desired folder.

The archive files contain an executable.

After unpacking the archive, go to the directory with the executable and run the program as:  

```shell
reduce_image_size <source_folder> <destination_folder> [options]
```

Or, provide full path to the program.

Paths to the source and destination folders can be absolute or relative.

## Building the Application and Running it With cargo
This section applies in case you don't have an executable and need to build it.

It may depend on the OS. Namely, while installation of `nasm` is needed on Windows, it is not needed on macOS.
`nasm` doesn't support Apple silicon, but this crate works on macOS Sonoma 14.5 on Apple M2 Pro processor.
Linux hasn't been tested. Also, macOS on x86 architecture hasn't been tested.

The library and the application require:
- [CMake](https://cmake.org/download/)
- [nasm](https://www.nasm.us/), on Windows

Add `CMake` to the `PATH` environment variable.

Make sure to build the application in `release` mode as it will run much faster that way.

Build:
```shell
cargo build --release
```

Run:
```shell
cargo run --release -- <source_folder> <destination_folder> [options]
```

## Library
This Rust crate was originally meant as a binary (executable) crate, i.e., an application,
but it was later decided to publish the library part, so it can be used as a Rust library, too.

Only the main image-processing function, `process_images`, has been made public.

Helper functions have been made private.
