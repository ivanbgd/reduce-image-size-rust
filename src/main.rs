//! Reduces size of images in a folder (and optionally sub-folders, recursively).
//!
//! The binary (executable) crate.

use std::path::Path;
use std::time::Instant;

use clap::Parser;

use reduce_image_size::cli::Args;
use reduce_image_size::logic::process_images;

/// The program's entry point.
///
/// Parses CLI arguments, calls the image processing function,
/// and in the end prints the total execution time.
fn main() {
    let start = Instant::now();

    let args = Args::parse();
    let src_dir = args.src_dir;
    let dst_dir = args.dst_dir;
    let recursive = args.recursive;
    let resize = args.resize;
    let quality = args.quality;

    if Path::new(&dst_dir).is_file() {
        println!(
            "\"{}\" exists and is a file! Provide a proper target directory.",
            dst_dir.display()
        );
        return;
    }

    process_images(src_dir, dst_dir, recursive, resize, quality);

    println!("\nTook {:.3?} to complete.", start.elapsed());
}
