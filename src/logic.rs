use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;

use globset::{GlobBuilder, GlobMatcher};
use oxipng::{optimize, InFile, Options, OutFile};
use pathdiff::diff_paths;
use walkdir::WalkDir;

use crate::constants::PATTERNS;

// fn get_glob() -> GlobMatcher {
//     GlobBuilder::new(PATTERNS)
//         .case_insensitive(true)
//         .build()
//         .unwrap()
//         .compile_matcher()
// }

/// Returns an iterator over the list of files under the `src_dir`, recursively or not.
/// Doesn't return subdirectories, but only files.
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

fn different_paths(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    recursive: bool,
    resize: bool,
    quality: i32,
) {
    // let glob = get_glob();
    let mut lock = stdout().lock();

    for src_path in get_file_list(&src_dir, recursive) {
        if let Some(extension) = src_path.path().extension() {
            // println!("{:?}", extension.to_string_lossy().to_lowercase()); // ///
            match extension.to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => {
                    let dst_path = dst_dir.as_path().join(
                        diff_paths(src_path.path().to_str().unwrap(), src_dir.to_str().unwrap())
                            .unwrap(),
                    );
                    fs::create_dir_all(dst_path.parent().unwrap()).unwrap();

                    let jpeg_data = fs::read(src_path.path()).unwrap();
                    let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data).unwrap();
                    // let scaled = img.resize(400, 400, filter);
                    let jpeg_data =
                        turbojpeg::compress_image(&image, quality, turbojpeg::Subsamp::Sub2x2)
                            .unwrap();
                    fs::write(&dst_path, &jpeg_data).unwrap();

                    writeln!(
                        lock,
                        "Resized \"{}\" to \"{}\".",
                        src_path.path().display(),
                        dst_path.display()
                    )
                    .expect("Failed to write to stdout.");
                }

                "png" => {
                    let dst_path = dst_dir.as_path().join(
                        diff_paths(src_path.path().to_str().unwrap(), src_dir.to_str().unwrap())
                            .unwrap(),
                    );
                    fs::create_dir_all(dst_path.parent().unwrap()).unwrap();

                    let options = Options::default();
                    // match optimize(&InFile::Path(src_path.into_path()), &OutFile::Path { path: Some(dst_path), preserve_attrs: true }, &options) {
                    match optimize(
                        &InFile::Path(src_path.clone().into_path()),
                        &OutFile::from_path(dst_path.clone()),
                        &options,
                    ) {
                        Ok(_) => writeln!(
                            lock,
                            "Resized \"{}\" to \"{}\".",
                            src_path.path().display(),
                            dst_path.display()
                        )
                        .expect("Failed to write to stdout."),
                        Err(err) => writeln!(lock, "{}", err).expect("Failed to write to stdout."),
                    };
                }

                _ => (),
            }
        }
    }
}

fn same_paths(src_dir: PathBuf, recursive: bool, resize: bool, quality: i32) {
    // let glob = get_glob();
    let mut lock = stdout().lock();
}

pub fn process_images(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    recursive: bool,
    resize: bool,
    quality: i32,
) {
    println!("JPEG quality = {quality}\n");
    stdout().flush().expect("Failed to flush stdout.");

    if src_dir != dst_dir {
        different_paths(src_dir, dst_dir, recursive, resize, quality);
    } else {
        same_paths(src_dir, recursive, resize, quality);
    }
}
