use std::fs::File;
use std::io::Cursor;
use std::io::{stdout, Write};
use std::path::PathBuf;

// use image::codecs::jpeg::JpegEncoder;
// use image::io::Reader as ImageReader;
// use image::{GenericImageView, ImageBuffer, ImageDecoder, ImageEncoder, ImageFormat};
// use image::RgbImage;
use glob::{glob, glob_with, MatchOptions};
use globset::{Glob, GlobBuilder, GlobSetBuilder};
use walkdir::WalkDir;

fn different_paths(src_dir: PathBuf, dst_dir: PathBuf, recursive: bool, resize: bool, quality: u8) {
    const PATTERNS1: [&str; 3] = ["*.jpg", "*.jpeg", "*.png"];
    const PATTERNS2: &str = "*.{jpg,jpeg,png}";
    const PATTERNS_REC: [&str; 3] = ["**/*.jpg", "**/*.jpeg", "**/*.png"];
    const PATTERNS_NON_REC: [&str; 3] = ["*.jpg", "*.jpeg", "*.png"];
    const PATTERN_REC: &str = "**/*.jp*g";
    const PATTERN_NON_REC: &str = "*.jp*g";

    // Recursive
    // for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
    for entry in WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        println!("{}", entry.path().display()); // ///
    }
    println!();

    // Non-recursive
    for entry in WalkDir::new(&src_dir)
        .min_depth(0)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        println!("{}", entry.path().display()); // ///
    }
    println!();

    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    // Recursive
    for entry in glob_with(
        format!("{}/{}", src_dir.display(), PATTERN_REC).as_str(),
        options,
    )
    .unwrap()
    .filter_map(Result::ok)
    .filter(|e| e.is_file())
    {
        println!("{}", entry.display());
    }
    println!();

    // Non-recursive
    for entry in glob_with(
        format!("{}/{}", src_dir.display(), PATTERN_NON_REC).as_str(),
        options,
    )
    .unwrap()
    .filter_map(Result::ok)
    .filter(|e| e.is_file())
    {
        println!("{}", entry.display());
    }
    println!();

    // Recursive
    let mut builder = GlobSetBuilder::new();
    for pattern in PATTERNS1 {
        builder.add(
            GlobBuilder::new(pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
        );
    }
    let glob_set = builder.build().unwrap();
    for entry in WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if glob_set.is_match(entry.path()) {
            println!("{}", entry.path().display()); // ///
        }
    }
    println!();

    // Recursive
    let _glob = GlobBuilder::new(PATTERNS2)
        .case_insensitive(true)
        .build()
        .unwrap()
        .compile_matcher();
    for entry in WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if _glob.is_match(entry.path()) {
            println!("{}", entry.path().display()); // ///
        }
    }
    println!();

    // let cont = WalkDir::new(src_dir);
    // println!(
    //     "{:#?}",
    //     cont.into_iter().filter_map(|e| e.ok()).collect::<Vec<_>>()
    // );

    // // TODO: Don't collect! Also, convert to Path!
    // let src_paths = match recursive {
    //     true => WalkDir::new(src_dir)
    //         .into_iter()
    //         .filter_map(|e| e.ok())
    //         .collect::<Vec<_>>(),
    //     false => vec![],
    // };
    //
    // let mut lock = stdout().lock();
    //
    // for src_path in src_paths {
    //     // println!("{}", src_path.path().display()); // remove
    //     writeln!(lock, "{}", src_path.path().display()).expect("Failed to write to stdout."); // remove
    //     let dst_path = dst_dir.as_path(); //.join(src_path.path()); // / src_path.path(); // fix
    //                                       // println!(
    //                                       //     "Resized \"{}\" to \"{}\".",
    //                                       //     src_path.path().display(),
    //                                       //     dst_path.display()
    //                                       // );
    //
    //     // let img = image::open("tests/images/jpg/progressive/cat.jpg").unwrap();
    //     // println!("dimensions {:?}", img.dimensions());
    //     // println!("{:?}", img.color());
    //     // img.save("test.png").unwrap();
    //
    //     writeln!(
    //         lock,
    //         "Resized \"{}\" to \"{}\".",
    //         src_path.path().display(),
    //         dst_path.display()
    //     )
    //     .expect("Failed to write to stdout.");
    // }

    // let src_path = "c:/sl/Slike/Lj/IMG_20220620_174403.jpg";
    // // let src_path = "c:/sl/Slike/pngs/pngwing.com.png";
    //
    // let jpeg_data = std::fs::read(src_path).unwrap();
    // let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data).unwrap();
    // let jpeg_data = turbojpeg::compress_image(&image, 75, turbojpeg::Subsamp::Sub2x2).unwrap();
    // std::fs::write(
    //     std::env::temp_dir().join("c:/dst/KopijeSlika/test2.jpg"),
    //     &jpeg_data,
    // )
    // .unwrap();
}

fn same_paths(src_dir: PathBuf, recursive: bool, resize: bool, quality: u8) {}

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
