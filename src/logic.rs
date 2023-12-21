use std::io::{stdout, Write};
use std::path::PathBuf;

use globset::{GlobBuilder, GlobMatcher};
use pathdiff::diff_utf8_paths;
use walkdir::WalkDir;

use crate::constants::PATTERNS;

fn get_glob() -> GlobMatcher {
    GlobBuilder::new(PATTERNS)
        .case_insensitive(true)
        .build()
        .unwrap()
        .compile_matcher()
}

fn get_file_list(src_dir: &PathBuf, recursive: bool) -> impl Iterator<Item = walkdir::DirEntry> {
    match recursive {
        true => WalkDir::new(src_dir).into_iter().filter_map(Result::ok),
        false => WalkDir::new(src_dir)
            .min_depth(0)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok),
    }
    .filter(|entry| entry.file_type().is_file())
}

fn different_paths(src_dir: PathBuf, dst_dir: PathBuf, recursive: bool, resize: bool, quality: u8) {
    let glob = get_glob();
    let mut lock = stdout().lock();

    for src_path in get_file_list(&src_dir, recursive) {
        if glob.is_match(src_path.path()) {
            let dst_path = dst_dir.as_path().join(
                diff_utf8_paths(src_path.path().to_str().unwrap(), src_dir.to_str().unwrap())
                    .unwrap(),
            );

            writeln!(
                lock,
                "Resized \"{}\" to \"{}\".",
                src_path.path().display(),
                dst_path.display()
            )
            .expect("Failed to write to stdout.");
        }
    }

    // let src_path = "c:/sl/Slike/Lj/IMG_20220620_174403.jpg";
    // // let src_path = "c:/sl/Slike/pngs/pngwing.com.png";
    //
    // let jpeg_data = std::fs::read(src_path).unwrap();
    // let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data).unwrap();
    // let jpeg_data =
    //     turbojpeg::compress_image(&image, quality as i32, turbojpeg::Subsamp::Sub2x2).unwrap();
    // std::fs::write(
    //     std::env::temp_dir().join("c:/dst/KopijeSlika/test2.jpg"),
    //     &jpeg_data,
    // )
    // .unwrap();
}

fn same_paths(src_dir: PathBuf, recursive: bool, resize: bool, quality: u8) {
    let glob = get_glob();
    let mut lock = stdout().lock();
}

pub fn process_images(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    recursive: bool,
    resize: bool,
    quality: u8,
) {
    println!("JPEG quality = {quality}\n");
    stdout().flush().expect("Failed to flush stdout.");

    if src_dir != dst_dir {
        different_paths(src_dir, dst_dir, recursive, resize, quality);
    } else {
        same_paths(src_dir, recursive, resize, quality);
    }
}
