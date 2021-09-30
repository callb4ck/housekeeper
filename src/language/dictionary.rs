use crate::language::{
    dictionary::DirType::*, // Forgive me, I'm lazy
    locales::{Locale, Locale::*},
};

pub enum DirType {
    Videos,
    Images,
    Packages,
    Documents,
    Presentations,
    Spreadsheets,
    ShellScripts,
    SourceFiles,
    Binaries,
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
            SourceFiles => "Sorgenti",
            Binaries => "Binari",
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
            SourceFiles => "Soule files",
            Binaries => "Binaries",
            Other => "Other",
        },
    }
    .to_string()
}
