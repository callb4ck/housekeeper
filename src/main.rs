mod dictionary;
mod locale;

use std::fs::{create_dir, read_dir, rename};
use dictionary::{
    DirType::*,
    get_dir
};

fn move_file(from: std::path::PathBuf, to: crate::dictionary::DirType) {
    let from = from.to_str().expect("Couldn't convert path");
    let mut to = get_dir(to);

    let _ = create_dir(to.clone());

    to.push_str(&from[1..]);

    println!("Moving {} to {}", from, to);

    rename(from, to).expect("Error moving file")
}

fn main() {
    for entry in read_dir(".").expect("Couldn't read contents of dir") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Ok(Some(kind)) = infer::get_from_path(&path) {
                let kind = kind.to_string();
                let kind = kind.split("/").collect::<Vec<&str>>()[0];

                move_file(
                    path,
                    match kind {
                        "image" => Images,
                        "video" => Videos,
                        _ => Other,
                    }
                )
            }
        }
    }
}
