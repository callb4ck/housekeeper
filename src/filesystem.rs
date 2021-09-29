use std::fs::{create_dir, rename};
use crate::dictionary::{get_dir, DirType};

pub fn move_file(from: std::path::PathBuf, to: DirType) {
    let from = from.to_str().expect("Couldn't convert path");
    let mut to = get_dir(to);

    let _ = create_dir(to.clone());

    to.push_str(&from[1..]);

    rename(from, to).expect("Error moving file")
}
