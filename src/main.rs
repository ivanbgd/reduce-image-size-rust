//! The program's entry point.

use clap::Parser;
use reduce_image_size::cli::Args;
use reduce_image_size::logic::process_images;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// The program's entry point.
fn main() {
    let start = Instant::now();

    let args = Args::parse();
    let src_dir = args.src_dir;
    let dst_dir = args.dst_dir;
    let recursive = args.recursive;
    let resize = args.resize;
    let quality = args.quality;

    // /// REMOVE
    println!(
        "{}, {}, {recursive}, {resize}, {quality}\n",
        src_dir.display(),
        dst_dir.display()
    );

    if Path::new(&dst_dir).is_file() {
        println!(
            "\"{}\" exists and is a file! Provide a proper target directory.",
            dst_dir.display()
        );
        return;
    }

    fs::create_dir_all(&dst_dir).unwrap();

    process_images(src_dir, dst_dir, recursive, resize, quality);

    println!("\nTook {:.3?} to complete.", start.elapsed());
}
