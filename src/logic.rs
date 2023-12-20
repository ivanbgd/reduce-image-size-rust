use std::fs::File;
use std::io::Cursor;
use std::io::{stdout, Write};
use std::path::PathBuf;

use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, ImageDecoder, ImageEncoder, ImageFormat};
// use walkdir::WalkDir;

fn different_paths(src_dir: PathBuf, dst_dir: PathBuf, recursive: bool, resize: bool, quality: u8) {
    // for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
    //     println!("{}", entry.path().display()); // ///
    // }

    // let cont = WalkDir::new(src_dir);
    // println!(
    //     "{:#?}",
    //     cont.into_iter().filter_map(|e| e.ok()).collect::<Vec<_>>()
    // );

    // //TODO: Try out `glob`!
    //
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

    let src_path = "c:/sl/Slike/Lj/IMG_20220620_174403.jpg";

    // let img = image::open(src_path).unwrap();
    // println!("dimensions {:?}", img.dimensions());
    // println!("{:?}", img.color());
    // img.save("c:/dst/KopijeSlika/test.jpg").unwrap();

    let mut default = vec![];
    let img = ImageReader::open(src_path).unwrap().decode().unwrap();
    // let img = ImageDecoder::read_image(default);
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
    // img.write_to(&mut Cursor::new(&mut default), ImageFormat::Jpeg).unwrap();

    let (w, h) = img.dimensions();
    let color = img.color();
    // let output = ImageBuffer::new(w, h).into_vec();
    // let mut buffer = File::create("c:/dst/KopijeSlika/test2.jpg").unwrap();
    // let mut writer = Write::write(&mut buffer, &mut output);
    // let encoder = JpegEncoder::new_with_quality(&mut writer, 70);

    let writer = &mut Cursor::new(&mut default);
    let encoder = JpegEncoder::new_with_quality(writer, 10);

    // encoder
    //     .encode(img.into_bytes().as_slice(), w, h, color)
    //     .unwrap();
    // encoder.encode_image(&img).unwrap();
    // encoder.write_image()

    img.write_with_encoder(encoder).unwrap();

    img.save("c:/dst/KopijeSlika/test2.jpg").unwrap();
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
