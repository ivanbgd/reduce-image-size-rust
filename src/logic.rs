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

fn read_file(src_path: &Path, lock: &mut StdoutLock) -> Option<Vec<u8>> {
    match fs::read(src_path) {
        Ok(data) => Some(data),
        Err(err) => {
            writeln!(
                lock,
                "\t[ERROR] Trying to read \"{}\" failed with the following error: {}.\n\
                 \tSkipping that file.",
                src_path.display(),
                err
            )
            .expect("Failed to write to stdout.");
            None
        }
    }
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
    // ) -> Result<Option<Vec<u8>>, std::io::Error> { ///
) -> Option<Vec<u8>> {
    // let src_path = Path::new("a.b"); // ///
    match resize {
        true => match resize_image(src_path) {
            Ok(data) => Some(data),
            Err(err) => {
                writeln!(
                    lock,
                    "\t[ERROR] Trying to resize \"{}\" failed with the following error: {}.\n\
                     \tWill attempt to reduce the image file size without resizing the image.",
                    src_path.display(),
                    err
                )
                .expect("Failed to write to stdout.");
                read_file(src_path, lock)
            }
        },
        false => read_file(src_path, lock),
    }
}

// TODO: Add error-handling.
fn process_jpeg(
    src_path: &Path,
    dst_path: &PathBuf,
    resize: bool,
    quality: i32,
    lock: &mut StdoutLock,
) -> bool {
    let image_data = match get_image_data(src_path, resize, lock) {
        Some(data) => data,
        None => return false,
    };

    let img: image::RgbaImage = turbojpeg::decompress_image(&image_data).unwrap();
    let optimized = turbojpeg::compress_image(&img, quality, turbojpeg::Subsamp::Sub2x2).unwrap();

    fs::write(dst_path, &optimized).unwrap();

    true
}

fn process_png(src_path: &Path, dst_path: &PathBuf, resize: bool, lock: &mut StdoutLock) -> bool {
    // ) -> Result<(), Box<dyn Error>> { ///
    let mut success = false;

    let image_data = match get_image_data(src_path, resize, lock) {
        Some(data) => data,
        None => return false,
    };

    match optimize_from_memory(&image_data, &Options::default()) {
        Ok(optimized) => match fs::write(dst_path, optimized) {
            Ok(_) => success = true,
            Err(err) => writeln!(
                lock,
                "\t[ERROR] Trying to write \"{}\" failed with the following error: {}.\n\
                 \tSkipping that file.",
                src_path.display(),
                err
            )
            .expect("Failed to write to stdout."),
        },
        Err(err) => writeln!(
            lock,
            "\t[ERROR] Trying to optimize \"{}\" failed with the following error: {}.\n\
             \tSkipping that file.",
            src_path.display(),
            err
        )
        .expect("Failed to write to stdout."),
    }

    success
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

    let different_paths = src_dir != dst_dir;
    let mut lock = stdout().lock();

    for src_path in get_file_list(&src_dir, recursive) {
        let src_path = src_path.path();
        if let Some(extension) = src_path.extension() {
            let mut dst_path = PathBuf::from(src_path); // TODO: Try with &Path!

            if different_paths {
                dst_path = dst_dir.as_path().join(
                    diff_paths(src_path.to_str().unwrap(), src_dir.to_str().unwrap()).unwrap(),
                );
                fs::create_dir_all(dst_path.parent().unwrap()).unwrap();
            }

            let mut success = false;

            match extension.to_string_lossy().to_lowercase().as_str() {
                "jpg" | "jpeg" => {
                    success = process_jpeg(src_path, &dst_path, resize, quality, &mut lock)
                }
                "png" => success = process_png(src_path, &dst_path, resize, &mut lock),
                _ => (),
            }

            if success {
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
        }
    }
}
