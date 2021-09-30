use crate::{
    filesystem::move_file, language::dictionary::DirType::*,
    parsers::mime_parsers::*,
};

pub fn handle_file(entry: Result<std::fs::DirEntry, std::io::Error>) {
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

