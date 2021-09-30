use crate::language::dictionary::{DirType, DirType::*};

use std::path::PathBuf;

/// Checks if string starts with any of the `&str`s
macro_rules! check_start {
    ($to_check:expr, $( $( $start:literal),* => $to_return:expr )* ) => {
        if false {None} // This is here to use else if later
        $(
            else if check_start!(@INTERNAL $to_check, $($start),*) {
                Some($to_return)
            }
        )*
        else {None}
    };

    (@INTERNAL $to_check:expr, $($start:literal),*) => {
        $(
            $to_check.starts_with($start)
        )||*
    }
}

/// Checks if string ends with any of the `&str`s
macro_rules! check_end {
    ($to_check:expr, $( $( $end:literal),* => $to_return:expr )* ) => {
        if false {None} // This is here to use else if later
        $(
            else if check_end!(@INTERNAL $to_check, $($end),*) {
                Some($to_return)
            }
        )*
        else {None}
    };

    (@INTERNAL $to_check:expr, $($start:literal),*) => {
        $(
            $to_check.ends_with($start)
        )||*
    }
}

/// Parse application/* mime-types
pub fn parse_application(mime: &str) -> DirType {
    match check_start!(
        mime,
        "vnd.openxmlformats-officedocument.wordprocessingml",
        "vnd.ms-word" => Documents

        "vnd.ms-powerpoint",
        "vnd.openxmlformats-officedocument.presentationml" => Presentations

        "vnd.ms-excel",
        "vnd.openxmlformats-officedocument.spreadsheetml" => Spreadsheets
    ) {
        Some(dir) => dir,

        None => match mime {
            "pdf" | "msword" => Documents,

            "vnd.debian.binary-package" | "x-rpm" => Packages,

            "x-executable" => Binaries,

            _ => Other,
        },
    }
}

/// Parse text/* mime-types
pub fn parse_text(mime: &str) -> Option<DirType> {
    match mime {
        "x-shellscript" => Some(ShellScripts),
        "x-c" => Some(SourceFiles),
        _ => None,
    }
}

/// Parse types that are not recognized by the infer crate
pub fn parse_unkown(path: &PathBuf) -> Option<DirType> {
    let path = path.to_str().expect("Invalid path");

    check_end!(path,
        ".md", ".tex", ".ltx" => Documents
        ".rs", ".go", ".java", ".c", ".cpp", ".cxx", ".cs", ".h", ".rb", ".py" => SourceFiles
        ".pyc", ".class" => Binaries
    )
}
