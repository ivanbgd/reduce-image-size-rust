use std::fs;
use std::io::BufWriter;
use std::io::{stdout, Write};
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};

use fast_image_resize as fr;
use globset::{GlobBuilder, GlobMatcher};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
use image::{DynamicImage, EncodableLayout, GenericImageView, ImageBuffer};
use oxipng::{optimize, optimize_from_memory, InFile, Options, OutFile};
use pathdiff::diff_paths;
// use png::{BitDepth, ColorType};
use walkdir::WalkDir;

use crate::constants::PATTERNS;

// fn get_glob() -> GlobMatcher {
//     GlobBuilder::new(PATTERNS)
//         .case_insensitive(true)
//         .build()
//         .unwrap()
//         .compile_matcher()
// }

// TODO: Add proper error-handling!

// TODO: Remove unused dependencies from this file and from Cargo.toml!
// Do it first for image-processing libs, and for globbing at the VERY end.

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

enum Encoder<W: Write> {
    Encoder(JpegEncoder<W>, PngEncoder<W>),
}

// TODO: Make it generic over JPEG & PNG!
// fn resize_image<W: Write>(src_path: &Path, enc: Encoder::<W>) -> Vec<u8> {
fn resize_image<T>(src_path: &Path) -> Vec<u8> {
    let img = ImageReader::open(src_path).unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();
    let color_type = img.color(); // TODO: Consider checking (matching by) `color_type`.

    let mut src_image = fr::Image::from_vec_u8(
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )
    .unwrap();

    println!(
        "{}, {}, {:?}, {:?}",
        width,
        height,
        color_type,
        src_image.pixel_type()
    ); // todo remove
    println!(
        "{:?}, {}, {}, {}, {}, {}",
        color_type,
        color_type.bytes_per_pixel(),
        color_type.bits_per_pixel(),
        color_type.has_alpha(),
        color_type.has_color(),
        color_type.channel_count()
    ); // todo comment-out

    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    let dst_width = NonZeroU32::new(width / 2).unwrap();
    let dst_height = NonZeroU32::new(height / 2).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    println!(
        "{}, {}, {:?}",
        dst_width,
        dst_height,
        dst_image.pixel_type()
    ); // todo remove

    let mut dst_view = dst_image.view_mut();

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    let mut result_buf = BufWriter::new(Vec::new());
    // JpegEncoder::new(&mut result_buf)
    T::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8, // color_type,
        )
        .unwrap();

    result_buf.into_inner().unwrap()
}

fn optimize_jpeg(jpeg_data: Vec<u8>, dst_path: &PathBuf, quality: i32) {
    // TODO: Consider checking (matching by) color type.
    let img: image::RgbaImage = turbojpeg::decompress_image(&jpeg_data).unwrap();
    let jpeg_data = turbojpeg::compress_image(&img, quality, turbojpeg::Subsamp::Sub2x2).unwrap();
    fs::write(dst_path, &jpeg_data).unwrap();
}

// TODO: Remove!
fn resize_png(src_path: &Path, dst_path: &PathBuf) {
    let mut img = image::open(src_path).unwrap();
    let (w, h) = img.dimensions();
    img = img.resize(w / 2, h / 2, FilterType::Lanczos3);
    img.save(dst_path).unwrap();
}

fn optimize_png() {}

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
            let dst_path = dst_dir
                .as_path()
                .join(diff_paths(src_path.to_str().unwrap(), src_dir.to_str().unwrap()).unwrap());
            fs::create_dir_all(dst_path.parent().unwrap()).unwrap();

            match extension.to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => {
                    // TODO: Consider adding `process_jpeg()`.
                    if resize {
                        let jpeg_data = resize_image::<JpegEncoder<dyn Write>>(src_path);
                        optimize_jpeg(jpeg_data, &dst_path, quality);
                    } else {
                        let jpeg_data = fs::read(src_path).unwrap();
                        optimize_jpeg(jpeg_data, &dst_path, quality);
                    }

                    writeln!(
                        lock,
                        "Resized \"{}\" to \"{}\".",
                        src_path.display(),
                        dst_path.display()
                    )
                    .expect("Failed to write to stdout.");
                }

                "png" => {
                    // TODO: Consider adding `process_png()`.
                    let mut new_src_path = Path::new(src_path);

                    if resize {
                        resize_png(src_path, &dst_path);
                        new_src_path = Path::new(&dst_path);
                    }

                    // TODO: Try to read from memory instead of writing to and reading again from a file!

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

// fn same_paths(src_dir: PathBuf, recursive: bool, resize: bool, quality: i32) {
//     // let glob = get_glob();
//     let mut lock = stdout().lock();
// }

// TODO: Consider removing `different_paths()` and `same_paths()`, and doing everything in `process_images()`.
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
        // same_paths(src_dir, recursive, resize, quality);
    }
}
