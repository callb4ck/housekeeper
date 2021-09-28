use crate::locale::Locale;
use crate::locale::Locale::*;


#[allow(dead_code)]
pub enum DirType {
    Videos,
    Images,
    Other,
}


#[allow(unused_macros)]
macro_rules! dict {
    ($dir_to_search:expr;
        $($locale:expr => $($generic_dir:expr, $literal_dir:expr;).*)*
    ) => {
        match Locale::from_env() {
            $(
                $locale => {
                    match $dir_to_search {
                        $(
                            stringify!($generic_dir) => $literal_dir
                        )*
                    }
                }
            )*,
        }
    }
}

#[allow(dead_code)]
pub fn get_dir(generic_dir: DirType) -> String {
    match Locale::from_env() {
        It => {
            match generic_dir {
                DirType::Videos => "Video",
                DirType::Images => "Immagini",
                DirType::Other => "Altro"
            }
        },

        _ => {
            match generic_dir {
                DirType::Videos => "Video",
                DirType::Images => "Images",
                DirType::Other => "Other"
            }
        }
    }.to_string()
}

