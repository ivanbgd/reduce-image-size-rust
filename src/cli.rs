use crate::constants::QUALITY;
use clap::Parser;
use std::path::PathBuf;

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

    /// JPEG quality, on a scale from 0 (worst) to 95 (best);
    /// ignored in case of PNGs
    #[arg(short, long, default_value_t = QUALITY,
    value_parser = clap::value_parser!(u8).range(0..=95))]
    pub quality: u8,
}
