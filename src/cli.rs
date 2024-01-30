//! The CLI arguments parser.

use std::path::PathBuf;

use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

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

    /// A minimum file size for which to perform file size reduction;
    /// DEFAULT = 0, S = 100 kB, M = 500 kB, L = 1 MB
    #[arg(short, long, default_value_t = SizeCLI::DEFAULT)]
    pub size: SizeCLI,
}

/// A minimum file size for which to perform file size reduction;
/// DEFAULT = 0, S = 100 kB, M = 500 kB, L = 1 MB
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SizeCLI {
    DEFAULT,
    S,
    M,
    L,
}

impl SizeCLI {
    /// Report all `possible_values`.
    pub fn possible_values() -> impl Iterator<Item = PossibleValue> {
        Self::value_variants()
            .iter()
            .filter_map(ValueEnum::to_possible_value)
    }
}

impl Default for SizeCLI {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl std::fmt::Display for SizeCLI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("No values are skipped.")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for SizeCLI {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {s}"))
    }
}

impl ValueEnum for SizeCLI {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::DEFAULT, Self::S, Self::M, Self::L]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::DEFAULT => PossibleValue::new("DEFAULT"),
            Self::S => PossibleValue::new("S").alias("s"),
            Self::M => PossibleValue::new("M").alias("m"),
            Self::L => PossibleValue::new("L").alias("l"),
        })
    }
}
