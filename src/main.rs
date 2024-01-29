//! # Reduce Image Size
//!
//! Reduces size of images in a folder (and optionally sub-folders, recursively).
//!
//! The binary (executable) crate.

use std::io::{stdout, Write};
use std::path::Path;
use std::time::Instant;

use clap::Parser;

use reduce_image_size::cli::{Args, SizeCLI};
use reduce_image_size::constants::Size;
use reduce_image_size::logic::process_images;

/// The program's entry point.
///
/// Parses CLI arguments, prints program configuration, calls the image processing function,
/// and in the end prints the total execution time and potential warning about errors.
fn main() {
    let start = Instant::now();

    let args = Args::parse();
    let src_dir = args.src_dir;
    let dst_dir = args.dst_dir;
    let recursive = args.recursive;
    let resize = args.resize;
    let quality = args.quality;

    let size = match args.size {
        SizeCLI::DEFAULT => Size::DEFAULT,
        SizeCLI::S => Size::S,
        SizeCLI::M => Size::M,
        SizeCLI::L => Size::L,
    } as u64;

    if Path::new(&dst_dir).is_file() {
        println!(
            "\"{}\" exists and is a file! Provide a proper target directory.",
            dst_dir.display()
        );
        return;
    }

    println!("Process recursively: {recursive}");
    println!("Reduce image dimensions: {resize}");
    println!("Minimum image file size for processing is {size:?} bytes.");
    println!("JPEG quality = {quality}\n");
    stdout().flush().expect("Failed to flush stdout.");

    let has_error = process_images(src_dir, dst_dir, recursive, resize, quality, size);

    println!("\nTook {:.3?} to complete.", start.elapsed());

    if has_error {
        println!("\nThere were some ERRORS and some files were skipped.");
        println!("It was not possible to reduce their size OR to copy them.");
        println!("Please review the [ERROR] messages so you don't potentially lose those files.\n");
    }
}
