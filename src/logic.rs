use std::fs;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::PathBuf;

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
        let mut src_path = src_path.path();
        if let Some(extension) = src_path.extension() {
            match extension.to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => {
                    let dst_path = dst_dir.as_path().join(
                        diff_paths(src_path.to_str().unwrap(), src_dir.to_str().unwrap()).unwrap(),
                    );
                    fs::create_dir_all(dst_path.parent().unwrap()).unwrap();

                    //
                    // let img = image::open(src_path).unwrap();
                    // let (w, h) = img.dimensions();
                    // let scaled = img.resize(w / 2, h / 2, FilterType::Lanczos3);
                    // println!(
                    //     "{}, {}; {}, {}; {}, {}",
                    //     w,
                    //     h,
                    //     w / 2,
                    //     h / 2,
                    //     scaled.dimensions().0,
                    //     scaled.dimensions().1
                    // );
                    // let jpeg_data = scaled.into_rgb8();
                    // let header = turbojpeg::read_header(jpeg_data.as_bytes()).unwrap();
                    // println!("{:?}", header);
                    // //

                    let jpeg_data = fs::read(src_path).unwrap();
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
                    let dst_path = dst_dir.as_path().join(
                        diff_paths(src_path.to_str().unwrap(), src_dir.to_str().unwrap()).unwrap(),
                    );
                    fs::create_dir_all(dst_path.parent().unwrap()).unwrap();

                    // let contents = fs::read(src_path).unwrap();
                    // let img = image::open(src_path).unwrap(); // ///
                    // let (w, h) = img.dimensions();
                    // let i = img.into_bytes(); // ///
                    // let contents = match resize {
                    //     true => {
                    //         let img2 = img.resize(400, 400, FilterType::Lanczos3);
                    //         img2.as_bytes()
                    //     },
                    //     false => img.as_bytes(),
                    // };

                    // let a = ImageBuffer::from_raw(1, 1, contents).unwrap();
                    // let b = ImageBuffer::from_vec(1, 1, contents).unwrap();

                    if resize {
                        let decoder = png::Decoder::new(File::open(src_path).unwrap());
                        let mut reader = decoder.read_info().unwrap();
                        let info = reader.info();
                        let color_type = info.color_type;
                        let bit_depth = info.bit_depth;
                        let (w1, h1) = (info.width as usize, info.height as usize);
                        assert_eq!(BitDepth::Eight, bit_depth);
                        // println!("{:?}", info);
                        let mut src = vec![0; reader.output_buffer_size()];
                        reader.next_frame(&mut src).unwrap();
                        let (w2, h2) = (w1 / 2, h1 / 2);
                        let mut dst = vec![0u8; w2 * h2 * color_type.samples()];
                        match color_type {
                            ColorType::Grayscale => {
                                resize::new(w1, h1, w2, h2, Pixel::Gray8, Lanczos3)
                                    .unwrap()
                                    .resize(src.as_gray(), dst.as_gray_mut())
                                    .unwrap()
                            }
                            ColorType::Rgb => resize::new(w1, h1, w2, h2, Pixel::RGB8, Lanczos3)
                                .unwrap()
                                .resize(src.as_rgb(), dst.as_rgb_mut())
                                .unwrap(),
                            ColorType::Indexed => (),
                            ColorType::GrayscaleAlpha => (),
                            ColorType::Rgba => resize::new(w1, h1, w2, h2, Pixel::RGBA8, Lanczos3)
                                .unwrap()
                                .resize(src.as_rgba(), dst.as_rgba_mut())
                                .unwrap(),
                        };
                        let outfh = File::create(&dst_path).unwrap();
                        let mut encoder = png::Encoder::new(outfh, w2 as u32, h2 as u32);
                        encoder.set_color(color_type);
                        encoder.set_depth(bit_depth);
                        encoder
                            .write_header()
                            .unwrap()
                            .write_image_data(&dst)
                            .unwrap();
                        src_path = &*dst_path;
                    }

                    // match optimize_from_memory(&i, &Options::default()) {
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
                        &InFile::Path(src_path.to_path_buf()),
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
