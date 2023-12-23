use std::fs;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

use globset::{GlobBuilder, GlobMatcher};
use image::imageops::FilterType;
use image::{DynamicImage, EncodableLayout, GenericImageView, ImageBuffer};
use oxipng::{optimize, optimize_from_memory, InFile, Options, OutFile};
use pathdiff::diff_paths;
use png::{BitDepth, ColorType};
use resize::Pixel;
use resize::Type::Lanczos3;
use rgb::FromSlice;
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

fn resize_image(src_path: &Path, dst_path: &PathBuf) {
    let mut img = image::open(src_path).unwrap();
    let (w, h) = img.dimensions();
    img = img.resize(w / 2, h / 2, FilterType::Lanczos3);
    img.save(dst_path).unwrap();
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
        let src_path = src_path.path();
        if let Some(extension) = src_path.extension() {
            let mut new_src_path = Path::new(src_path);

            let dst_path = dst_dir
                .as_path()
                .join(diff_paths(src_path.to_str().unwrap(), src_dir.to_str().unwrap()).unwrap());
            fs::create_dir_all(dst_path.parent().unwrap()).unwrap();

            match extension.to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => {
                    if resize {
                        resize_image(src_path, &dst_path);
                        new_src_path = Path::new(&dst_path);
                    }

                    let jpeg_data = fs::read(new_src_path).unwrap();
                    let img: image::RgbImage = turbojpeg::decompress_image(&jpeg_data).unwrap();
                    let jpeg_data =
                        turbojpeg::compress_image(&img, quality, turbojpeg::Subsamp::Sub2x2)
                            .unwrap();
                    fs::write(&dst_path, &jpeg_data).unwrap();

                    writeln!(
                        lock,
                        "Resized \"{}\" to \"{}\".",
                        src_path.display(),
                        dst_path.display()
                    )
                    .expect("Failed to write to stdout.");
                }

                "png" => {
                    if resize {
                        resize_image(src_path, &dst_path);
                        new_src_path = Path::new(&dst_path);
                    }

                    // let contents = fs::read(src_path).unwrap();

                    // match optimize_from_memory(&contents, &Options::default()) {
                    //     Ok(optimized) => {
                    //         fs::write(&dst_path, optimized).unwrap();
                    //         writeln!(
                    //             lock,
                    //             "Resized \"{}\" to \"{}\".",
                    //             src_path.display(),
                    //             dst_path.display()
                    //         )
                    //         .expect("Failed to write to stdout.")
                    //     }
                    //     Err(err) => writeln!(lock, "{}", err).expect("Failed to write to stdout."),
                    // }

                    match optimize(
                        &InFile::Path(new_src_path.to_path_buf()),
                        &OutFile::from_path(dst_path.clone()),
                        &Options::default(),
                    ) {
                        Ok(_) => writeln!(
                            lock,
                            "Resized \"{}\" to \"{}\".",
                            src_path.display(),
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
