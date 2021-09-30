mod core;
mod filesystem;
mod language;
mod parsers;

use crate::{core::handle_file, parsers::arg_parser};

use std::{env::set_current_dir, fs::read_dir, thread::spawn};

/*
 * Mime type lists:
 * https://www.iana.org/assignments/media-types/media-types.xhtml
 *
 * https://stackoverflow.com/questions/4212861/what-is-a-correct-mime-type-for-docx-pptx-etc
 */

fn main() {
    let opts = arg_parser::parse();

    if let Some(path) = opts.path {
        set_current_dir(path).expect("Error while opening path");
    }

    let dir = read_dir(".").expect("Couldn't read contents of dir");
    if opts.single_thread {
        for entry in dir {
            handle_file(entry);
        }
    } else {
        let dir: Vec<_> = dir.collect();

        let mut join_handles = Vec::with_capacity(dir.len());
        for entry in dir {
            join_handles.push(spawn(move || handle_file(entry)));
        }

        for handle in join_handles {
            handle.join().expect("Thread panicked!");
        }
    }
}
