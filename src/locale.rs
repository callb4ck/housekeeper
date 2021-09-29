pub enum Locale {
    En,
    It,
}

impl Locale {
    /// Get locale from LANG environment variable.
    pub fn from_env() -> Locale {
        match std::env::var("LANG") {
            Ok(locale) => {
                let locale = &locale[..5];

                match locale {
                    "it_IT" => Locale::It,
                    _ => Locale::En,
                }
            }

            _ => Locale::En,
        }
    }
}
