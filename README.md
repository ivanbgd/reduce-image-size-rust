# Reduce Image Size

[![CI](https://github.com/ivanbgd/reduce-image-size-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/ivanbgd/reduce-image-size-rust/actions/workflows/ci.yml)

## Description
Reduces size of images in a folder (and optionally sub-folders, recursively).

Supports JPEG and PNG formats.

This is useful for archiving of photos, for example, as they look the same on a display even with a reduced file size.  
This application reduces the file sizes of the images in bulk.

By default, keeps the original images and creates copies with reduced file size.

By default, copies the entire folder tree, with all sub-folders that exist in the source tree.  
The target folder tree will be created automatically, and the new reduced-size images will be copied properly to their respective paths.  
It is only required to provide the root target folder, and it will also be created if it doesn't exist.

The destination folder can be the same as the source folder, in which case the original images will be **overwritten**, and not retained.

If there is enough disk space, it is advised to specify a different destination folder than the source folder,
so that the original images can be retained and the newly-created reduced-size images can be inspected for quality.  
A user can experiment with the `resize` and the `quality` arguments.  
Also, the user can go only one level deep and not recursively, or simply experiment on a copy of an image folder.  
If satisfied with the result, original images can be deleted afterwards easily to save disk and/or cloud space.

## Options
- Look into subdirectories recursively (process the entire tree); recommended: `-r`, `--recursive`
- Reduce both image dimensions by half: `--resize`
- JPEG quality, on a scale from 1 (worst) to 100 (best); the default is 75; ignored in case of PNGs: `--quality <QUALITY>`

## Notes
- Developed in Rust 1.74.1.
- Tested on x86-64 CPUs with Windows 10 and Windows 11.
- Also tested on WSL - Ubuntu 22.04.2 LTS (GNU/Linux 5.15.133.1-microsoft-standard-WSL2 x86_64) on Windows 11.
- Other OSes haven't been tested, but should work.

## Running the Application
Executable files for Windows, Linux and macOS can be downloaded from
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
It doesn't depend on the OS.

The library and the application require:
- [CMake](https://cmake.org/download/)
- [nasm](https://www.nasm.us/)

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

Only the main image processing function, `process_images`, has been made public.

Helper functions have been made private.
