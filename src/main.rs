//! The program's entry point.

use clap::Parser;
use reduce_image_size::cli::Args;
use reduce_image_size::logic::process_images;
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
        "{}, {}, {recursive}, {resize}, {quality}",
        src_dir.display(),
        dst_dir.display()
    );

    process_images();

    println!("\nTook {:.3?} to complete.", start.elapsed());
}
