mod dictionary;
mod locale;
mod filesystem;
mod mimeparsers;

use std::fs::read_dir;
use std::thread::spawn;

use crate::{
    dictionary::DirType::*,
    filesystem::move_file,
    mimeparsers::*
};

/*
 * Mime type lists:
 * https://www.iana.org/assignments/media-types/media-types.xhtml
 *
 * https://stackoverflow.com/questions/4212861/what-is-a-correct-mime-type-for-docx-pptx-etc
 */

const USE_THREADS: bool = true;

fn handle_file(entry: Result<std::fs::DirEntry, std::io::Error>) {
    if let Ok(entry) = entry {
        let path = entry.path();
        if let Ok(Some(kind)) = infer::get_from_path(&path) {
            let kind = kind.to_string();

            print!(
                "\nPath:\t{}\nType:\t{}\n\n",
                path.to_str().unwrap_or("Error"),
                kind
            );

            let kind_collection = kind.split("/").collect::<Vec<&str>>();
            let kind = kind_collection[0];

            move_file(
                path,
                match kind {
                    "image" => Images,
                    "video" => Videos,
                    "application" => parse_application(kind_collection[1]),
                    "text" => parse_text(kind_collection[1]),
                    _ => Other,
                },
            );
        }
    }
}

fn main() {
    let dir = read_dir(".").expect("Couldn't read contents of dir");
    if USE_THREADS {
        let dir: Vec<_> = dir.collect();

        let mut join_handles = Vec::with_capacity(dir.len());
        for entry in dir {
            join_handles.push(spawn(move || handle_file(entry)));
        }

        for handle in join_handles {
            handle.join().expect("Thread panicked!");
        }
    } else {
        for entry in dir {
            handle_file(entry);
        }
    }
}
