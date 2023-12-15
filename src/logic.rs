use std::io::{stdout, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

fn different_paths(src_dir: PathBuf, dst_dir: PathBuf, recursive: bool, resize: bool, quality: u8) {
    // for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
    //     println!("{}", entry.path().display()); // ///
    // }

    // let cont = WalkDir::new(src_dir);
    // println!(
    //     "{:#?}",
    //     cont.into_iter().filter_map(|e| e.ok()).collect::<Vec<_>>()
    // );

    // TODO: Don't collect! Also, convert to Path!
    let src_paths = match recursive {
        true => WalkDir::new(src_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>(),
        false => vec![],
    };

    for src_path in src_paths {
        println!("{}", src_path.path().display()); // remove
        let dst_path = dst_dir.as_path(); //.join(src_path.path()); // / src_path.path(); // fix
        println!(
            "Resized \"{}\" to \"{}\".",
            src_path.path().display(),
            dst_path.display()
        );
        stdout().flush().expect("Failed to flush stdout.");
    }
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
