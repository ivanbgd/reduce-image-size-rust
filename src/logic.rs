//! The main business logic (public) and helper functions (private).

use std::error::Error;
use std::fs;
use std::io::{BufWriter, stdout, StdoutLock, Write};
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};

use fast_image_resize as fr;
use image::{ColorType, ImageEncoder};
use image::codecs::{jpeg::JpegEncoder, png::PngEncoder};
use image::io::Reader as ImageReader;
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
        NonZeroU32::new(width).expect("Expected NonZeroU32."),
        NonZeroU32::new(height).expect("Expected NonZeroU32."),
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )?;

    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;

    let dst_width = NonZeroU32::new(width / 2).expect("Expected NonZeroU32.");
    let dst_height = NonZeroU32::new(height / 2).expect("Expected NonZeroU32.");
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
/// Varies the message contents depending on whether the source and
/// destination paths are same or different.
#[inline]
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

/// Sets the flag `has_error`. Prints an error message to `stdout`.
///
/// Wraps around the received error message,
/// and notifies the end user that the image file will be skipped.
#[inline]
fn set_and_print_error(
    src_path: &Path,
    err: Box<dyn Error>,
    lock: &mut StdoutLock,
    has_error: &mut bool,
) {
    *has_error = true;

    writeln!(
        lock,
        "\t[ERROR] Trying to reduce size of \"{}\" failed with the following error: {}.\n\
         \tSkipping that file.\n",
        src_path.display(),
        err
    )
    .expect("Failed to write to stdout.");
}

/// Copies a file in case of different source and destination paths.
///
/// Skips a file in case of same source and destination path.
///
/// Prints an info message in either case.
fn copy_or_skip(
    src_path: &Path,
    dst_path: &Path,
    different_paths: bool,
    lock: &mut StdoutLock,
    err: Option<Box<dyn Error>>,
    has_error: &mut bool,
) {
    if let Some(error) = err {
        writeln!(lock, "{}", error).expect("Failed to write to stdout.");
    };

    if different_paths {
        match fs::copy(src_path, dst_path) {
            Ok(_) => writeln!(
                lock,
                "Copied \"{}\" to \"{}\".",
                src_path.display(),
                dst_path.display()
            )
            .expect("Failed to write to stdout."),
            Err(e) => set_and_print_error(src_path, Box::from(e), lock, has_error),
        };
    } else {
        writeln!(lock, "Skipped \"{}\".", src_path.display()).expect("Failed to write to stdout.");
    }
}

/// The main business logic.
/// Loops over files and calls appropriate functions for processing images.
/// Processing consists of optional resizing first, and of optimizing images
/// in order to reduce the file size.
/// Supported image formats: JPEG, PNG.
///
/// * `src_dir` - Source directory path with the original images, [`PathBuf`].
/// * `dst_dir` - Destination directory path with the reduced-size images, [`PathBuf`].
/// * `recursive` - Whether to look into entire directory subtree.
/// * `resize` - Whether to resize image dimensions.
/// * `quality` - JPEG image quality. Ignored in case of PNGs.
///
/// Returns `bool` stating whether there was any error in trying to reduce size of a file or to copy it.
/// This `bool` can be `true` only in case where source and destination directories are different,
/// because in case where they are same and a file cannot have its size reduced, it will be left intact
/// in its source directory.
pub fn process_images(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    recursive: bool,
    resize: bool,
    quality: i32,
    size: u64,
) -> bool {
    let mut has_error = false;

    let different_paths = src_dir != dst_dir;

    // The `lock` is used in combination with `writeln!` for printing to `stdout` in a loop.
    // This is faster than `println!` in a hot loop, because we now only lock `stdout` once.
    let mut lock = stdout().lock();

    for src_path in get_file_list(&src_dir, recursive) {
        let src_path = src_path.path();

        let mut dst_path = PathBuf::from(src_path);

        if different_paths {
            dst_path = dst_dir.as_path().join(
                diff_paths(
                    src_path.to_str().expect("Expected some src_path."),
                    src_dir.to_str().expect("Expected some src_dir."),
                )
                .expect("Expected diff_paths() to work."),
            );

            if let Some(parent) = dst_path.parent() {
                match fs::create_dir_all(parent) {
                    Ok(_) => {}
                    Err(err) => {
                        let err = format!(
                            "\n\tFailed to create the subdirectory {:?} with the following error: {}",
                            parent, err
                        );
                        set_and_print_error(src_path, Box::from(err), &mut lock, &mut has_error);
                        continue;
                    }
                };
            } else {
                let err_msg = format!("Destination path {:?} doesn't have a parent.", dst_path);
                set_and_print_error(src_path, Box::from(err_msg), &mut lock, &mut has_error);
                continue;
            };
        }

        let file_size = src_path.metadata().expect("Expected file metadata.").len();
        let extension = src_path.extension();

        // Copy or skip a file if it is not large enough, or has no extension, or if its extension is not supported.
        if file_size >= size && extension.is_some() {
            match extension.unwrap().to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => {
                    match process_jpeg(src_path, &dst_path, resize, quality, &mut lock) {
                        Ok(_) => print_success(src_path, &dst_path, different_paths, &mut lock),
                        Err(err) => copy_or_skip(
                            src_path,
                            &dst_path,
                            different_paths,
                            &mut lock,
                            Some(err),
                            &mut has_error,
                        ),
                    }
                }
                "png" => match process_png(src_path, &dst_path, resize, &mut lock) {
                    Ok(_) => print_success(src_path, &dst_path, different_paths, &mut lock),
                    Err(err) => copy_or_skip(
                        src_path,
                        &dst_path,
                        different_paths,
                        &mut lock,
                        Some(err),
                        &mut has_error,
                    ),
                },
                _ => copy_or_skip(
                    src_path,
                    &dst_path,
                    different_paths,
                    &mut lock,
                    None,
                    &mut has_error,
                ),
            }
        } else {
            copy_or_skip(
                src_path,
                &dst_path,
                different_paths,
                &mut lock,
                None,
                &mut has_error,
            );
        }
    }

    has_error
}
