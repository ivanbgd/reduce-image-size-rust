//! The program's entry point.

use clap::Parser;
use reduce_image_size::cli::Args;
// use reduce_image_size::logic::process_images;

/// The program's entry point.
fn main() {
    let args = Args::parse();
    let src_dir = args.src_dir;
    let dst_dir = args.dst_dir;
    let recursive = args.recursive;
    let resize = args.resize;
    let quality = args.quality;

    println!(
        "{}, {}, {recursive}, {resize}, {quality}",
        src_dir.display(),
        dst_dir.display()
    );
}
