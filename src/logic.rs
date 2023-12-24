use std::error::Error;
use std::fs;
use std::io::{stdout, BufWriter, StdoutLock, Write};
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};

use fast_image_resize as fr;
use image::codecs::{jpeg::JpegEncoder, png::PngEncoder};
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
use oxipng::{optimize_from_memory, Options};
use pathdiff::diff_paths;
use walkdir::WalkDir;

// TODO: Add proper error-handling!

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

fn resize_image(src_path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = ImageReader::open(src_path)?
        .with_guessed_format()?
        .decode()?;
    let width = img.width();
    let height = img.height();

    let mut src_image = fr::Image::from_vec_u8(
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )?;

    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;

    let dst_width = NonZeroU32::new(width / 2).unwrap();
    let dst_height = NonZeroU32::new(height / 2).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    let mut dst_view = dst_image.view_mut();

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_image.view(), &mut dst_view)?;

    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    let mut result_buf = BufWriter::new(Vec::new());

    let extension = src_path
        .extension()
        .expect("Expected the file to have an extension at this point!");
    match extension.to_string_lossy().to_lowercase().as_str() {
        "jpg" | "jpeg" => JpegEncoder::new(&mut result_buf).write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8, // color_type,
        )?,
        "png" => PngEncoder::new(&mut result_buf).write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8,
        )?,
        _ => panic!("Unsupported image format (file extension): {:?}", extension),
    }

    let result = result_buf.into_inner()?;

    Ok(result)
}

// TODO: Add error-handling.
fn process_jpeg(
    src_path: &Path,
    dst_path: PathBuf,
    resize: bool,
    quality: i32,
    lock: &mut StdoutLock,
) {
    let image_data = match resize {
        true => match resize_image(src_path) {
            Ok(data) => data,
            Err(err) => {
                writeln!(
                    lock,
                    "\t[ERROR] Trying to resize \"{}\" failed with the following error: {}.\n\
                     \tWill attempt to optimize the image without resizing it.",
                    src_path.display(),
                    err
                )
                .expect("Failed to write to stdout.");
                match fs::read(src_path) {
                    Ok(data) => data,
                    Err(err) => {
                        writeln!(
                            lock,
                            "\t[ERROR] Trying to read \"{}\" failed with the following error: {}.",
                            src_path.display(),
                            err
                        )
                        .expect("Failed to write to stdout.");
                        return;
                    }
                }
            }
        },
        false => match fs::read(src_path) {
            Ok(data) => data,
            Err(err) => {
                writeln!(
                    lock,
                    "\t[ERROR] Trying to read \"{}\" failed with the following error: {}.",
                    src_path.display(),
                    err
                )
                .expect("Failed to write to stdout.");
                return;
            }
        },
    };

    let img: image::RgbaImage = turbojpeg::decompress_image(&image_data).unwrap();
    let optimized = turbojpeg::compress_image(&img, quality, turbojpeg::Subsamp::Sub2x2).unwrap();

    fs::write(&dst_path, &optimized).unwrap();

    // TODO: See if you can extract this writeln.
    writeln!(
        lock,
        "Reduced \"{}\" to \"{}\".",
        src_path.display(),
        dst_path.display()
    )
    .expect("Failed to write to stdout.");
}

// TODO: Add error-handling.
fn process_png(src_path: &Path, dst_path: PathBuf, resize: bool, lock: &mut StdoutLock) {
    let image_data = match resize {
        true => resize_image(src_path).unwrap(),
        false => vec![], //fs::read(src_path).unwrap(),
    };

    match optimize_from_memory(&image_data, &Options::default()) {
        Ok(optimized) => {
            fs::write(&dst_path, optimized).unwrap();
            writeln!(
                lock,
                "Reduced \"{}\" to \"{}\".",
                src_path.display(),
                dst_path.display()
            )
            .expect("Failed to write to stdout.")
        }
        Err(err) => writeln!(lock, "{}", err).expect("Failed to write to stdout."),
    }
}

fn different_paths(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    recursive: bool,
    resize: bool,
    quality: i32,
) {
    let mut lock = stdout().lock();

    for src_path in get_file_list(&src_dir, recursive) {
        let src_path = src_path.path();
        if let Some(extension) = src_path.extension() {
            let dst_path = dst_dir
                .as_path()
                .join(diff_paths(src_path.to_str().unwrap(), src_dir.to_str().unwrap()).unwrap());
            fs::create_dir_all(dst_path.parent().unwrap()).unwrap();

            // TODO: Consider adding `process_image()`.
            match extension.to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => process_jpeg(src_path, dst_path, resize, quality, &mut lock),
                "png" => process_png(src_path, dst_path, resize, &mut lock),
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
