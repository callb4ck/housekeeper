use crate::language::dictionary::{DirType, DirType::*};

/// Check if string starts with any of the &str
macro_rules! check_start {
    ($to_check:expr, $($start:literal),*) => {
        $(
            $to_check.starts_with($start) ||
        )* false // This is here to fix trailing || operator
    }
}

/// Parse application/* mime-types
pub fn parse_application(mime: &str) -> DirType {
    if check_start!(
        mime,
        "vnd.openxmlformats-officedocument.wordprocessingml",
        "vnd.ms-word"
    ) {
        Documents
    } else if check_start!(
        mime,
        "vnd.ms-powerpoint",
        "vnd.openxmlformats-officedocument.presentationml"
    ) {
        Presentations
    } else if check_start!(
        mime,
        "vnd.ms-excel",
        "vnd.openxmlformats-officedocument.spreadsheetml"
    ) {
        Spreadsheets
    } else {
        match mime {
            "pdf" | "msword" => Documents,

            "vnd.debian.binary-package" | "x-rpm" => Packages,

            "x-executable" => Binaries,

            _ => Other,
        }
    }
}

/// Parse text/* mime-types
pub fn parse_text(mime: &str) -> DirType {
    match mime {
        "x-shellscript" => ShellScripts,
        "x-c" => SourceFiles,

        _ => Other,
    }
}
