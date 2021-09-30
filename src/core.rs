use crate::{
    filesystem::move_file,
    language::dictionary::{DirType, DirType::*},
    parsers::type_parsers::*,
};

use std::path::PathBuf;

/// Let the infer crate handle the type
fn handle_known(kind: infer::Type, path: &PathBuf) -> Option<DirType> {
    let kind = kind.to_string();


    let kind_collection = kind.split("/").collect::<Vec<&str>>();

    let return_kind = match kind_collection[0] {
        "image" => Images,
        "video" => Videos,
        "application" => parse_application(kind_collection[1]),
        "text" => parse_text(kind_collection[1])?,
        _ => Other,
    };

    print!(
        "\nPath:\t{}\nType:\t{}\n\n",
        path.to_str().unwrap_or("Error"),
        kind
    );

    Some(return_kind)
}

fn handle_unknown(path: &PathBuf) -> Option<DirType> {
    let kind = parse_unkown(path)?;

    print!(
        "\nPath:\t{}\nType:\t{:?}\n\n",
        path.to_str().unwrap_or("Error"),
        kind
    );

    Some(kind)
}

pub fn handle_file(entry: Result<std::fs::DirEntry, std::io::Error>, dry_run: bool) {
    if let Ok(entry) = entry {
        let path = entry.path();
        if let Ok(kind) = infer::get_from_path(&path) {
            let kind = match kind {
                Some(kind) => handle_known(kind, &path),

                None => handle_unknown(&path),
            };

            if let Some(kind) = kind {
                if !dry_run {
                    move_file(path, kind);
                }
            }
        }
    }
}
