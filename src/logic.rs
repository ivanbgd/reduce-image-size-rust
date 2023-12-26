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

/// Returns an iterator over the list of files under the `src_dir`, recursively or not.
/// Doesn't return subdirectories, but only files.
fn get_file_list(src_dir: &Path, recursive: bool) -> impl Iterator<Item = walkdir::DirEntry> {
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

/// Reduces the image dimensions in half.
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

/// In case `resize` is `true`, tries to resize the image.
/// If that succeeds, returns the resized image data.
///
/// If that fails, or if `resize` is `false`,
/// returns the original image data read from the image file.
fn get_image_data(
    src_path: &Path,
    resize: bool,
    lock: &mut StdoutLock,
) -> Result<Vec<u8>, std::io::Error> {
    match resize {
        true => match resize_image(src_path) {
            Ok(data) => Ok(data),
            Err(err) => {
                writeln!(
                    lock,
                    "\t[ERROR] Trying to resize \"{}\" failed with the following error: {}.\n\
                     \tWill attempt to reduce the file size of the image without resizing the image.",
                    src_path.display(),
                    err
                )
                .expect("Failed to write to stdout.");
                fs::read(src_path)
            }
        },
        false => fs::read(src_path),
    }
}

/// Reduces size of a JPEG image file.
///
/// Resizes the image first if that option was set.
/// Optimizes the image quality and file size.
fn process_jpeg(
    src_path: &Path,
    dst_path: &Path,
    resize: bool,
    quality: i32,
    lock: &mut StdoutLock,
) -> Result<(), Box<dyn Error>> {
    let image_data = get_image_data(src_path, resize, lock)?;

    let img: image::RgbaImage = turbojpeg::decompress_image(&image_data)?;
    let optimized = turbojpeg::compress_image(&img, quality, turbojpeg::Subsamp::Sub2x2)?;

    fs::write(dst_path, &optimized)?;

    Ok(())
}

/// Reduces size of a PNG image file.
///
/// Resizes the image first if that option was set.
/// Optimizes the image quality and file size.
fn process_png(
    src_path: &Path,
    dst_path: &Path,
    resize: bool,
    lock: &mut StdoutLock,
) -> Result<(), Box<dyn Error>> {
    let image_data = get_image_data(src_path, resize, lock)?;

    let optimized = optimize_from_memory(&image_data, &Options::default())?;

    fs::write(dst_path, optimized)?;

    Ok(())
}

/// Prints a success message to `stdout`.
///
/// Varies the message output depending on whether the source and
/// destination paths are same or different.
fn print_success(src_path: &Path, dst_path: &Path, different_paths: bool, lock: &mut StdoutLock) {
    match different_paths {
        true => writeln!(
            lock,
            "Reduced \"{}\" to \"{}\".",
            src_path.display(),
            dst_path.display()
        )
        .expect("Failed to write to stdout."),
        false => writeln!(lock, "Reduced \"{}\".", src_path.display())
            .expect("Failed to write to stdout."),
    }
}

/// Prints an error message to `stdout`.
///
/// Wraps around the received error message,
/// and notifies the end user that the image file will be skipped.
fn print_error(src_path: &Path, err: Box<dyn Error>, lock: &mut StdoutLock) {
    writeln!(
        lock,
        "\t[ERROR] Trying to reduce size of \"{}\" failed with the following error: {}.\n\
         \tSkipping that file.\n",
        src_path.display(),
        err
    )
    .expect("Failed to write to stdout.")
}

/// Loops over files and calls appropriate functions for processing images.
/// Processing consists of optional resizing first, and of optimizing images
/// in order to reduce the file size.
pub fn process_images(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    recursive: bool,
    resize: bool,
    quality: i32,
) {
    println!("JPEG quality = {quality}\n");
    stdout().flush().expect("Failed to flush stdout.");

    let different_paths = src_dir != dst_dir;
    let mut lock = stdout().lock();

    for src_path in get_file_list(&src_dir, recursive) {
        let src_path = src_path.path();
        if let Some(extension) = src_path.extension() {
            let mut dst_path = PathBuf::from(src_path);

            if different_paths {
                dst_path = dst_dir.as_path().join(
                    diff_paths(src_path.to_str().unwrap(), src_dir.to_str().unwrap()).unwrap(),
                );
                fs::create_dir_all(dst_path.parent().unwrap()).unwrap();
            }

            match extension.to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => {
                    match process_jpeg(src_path, &dst_path, resize, quality, &mut lock) {
                        Ok(_) => print_success(src_path, &dst_path, different_paths, &mut lock),
                        Err(err) => print_error(src_path, err, &mut lock),
                    }
                }
                "png" => match process_png(src_path, &dst_path, resize, &mut lock) {
                    Ok(_) => print_success(src_path, &dst_path, different_paths, &mut lock),
                    Err(err) => print_error(src_path, err, &mut lock),
                },
                _ => (),
            }
        }
    }
}
