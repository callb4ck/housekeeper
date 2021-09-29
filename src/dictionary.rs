use crate::locale::Locale;
use crate::locale::Locale::*;

use crate::dictionary::DirType::*; // Forgive me, I'm lazy

pub enum DirType {
    Videos,
    Images,
    Packages,
    Documents,
    Presentations,
    Spreadsheets,
    ShellScripts,
    Other,
}

pub fn get_dir(generic_dir: DirType) -> String {
    match Locale::from_env() {
        It => match generic_dir {
            Videos => "Video",
            Images => "Immagini",
            Packages => "Pacchetti",
            Documents => "Documenti",
            Presentations => "Presentazioni",
            Spreadsheets => "Fogli di calcolo",
            ShellScripts => "Script shell",
            Other => "Altro",
        },

        _ => match generic_dir {
            Videos => "Videos",
            Images => "Images",
            Packages => "Packages",
            Documents => "Documents",
            Presentations => "Presentations",
            Spreadsheets => "Spreadsheets",
            ShellScripts => "Shell scripts",
            Other => "Other",
        },
    }
    .to_string()
}
