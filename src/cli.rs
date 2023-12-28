//! The CLI arguments parser.

use std::path::PathBuf;

use clap::Parser;

use crate::constants::QUALITY;

/// Reduces size of images in a folder (and optionally sub-folders, recursively)
#[derive(Parser)]
#[command(name = "Reduce Image Size")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to source folder with original images
    pub src_dir: PathBuf,

    /// Path to destination folder for reduced-size copies of original images;
    /// can be the same as source, in which case source images are overwritten
    pub dst_dir: PathBuf,

    /// Look recursively in sub-folders
    #[arg(short, long)]
    pub recursive: bool,

    /// Reduce both image dimensions by half
    #[arg(long)]
    pub resize: bool,

    /// JPEG quality, on a scale from 1 (worst) to 100 (best);
    /// ignored in case of PNGs
    #[arg(short, long, default_value_t = QUALITY,
    value_parser = clap::value_parser!(i32).range(1..=100))]
    pub quality: i32,
}
