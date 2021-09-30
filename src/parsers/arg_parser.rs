use std::{env::args, process::exit};

#[derive(Default)]
pub struct ArgOptions {
    pub single_thread: bool,
    pub path: Option<String>,
}

fn help() {
    println!(
        "Use: housekeeper [OPTS] [PATH]
    -h,  --help:          Show this help message
    -st, --single-thread: Only use one thread

--- NOTE ---

If no path is given current working directory is used.
"
    );
    exit(0)
}

pub fn parse() -> ArgOptions {
    let mut opts = ArgOptions::default();
    let mut path = String::new();
    let mut arg_list = args();

    arg_list.next(); // Skip binary path

    for arg in arg_list {
        match arg.to_lowercase().as_str() {
            "-st" | "--single-thread" => opts.single_thread = true,
            "-h" | "--help" => help(),
            _ => {
                if !arg.starts_with("-") {
                    path.push_str(arg.as_str());
                    path.push(' ');
                }
            }
        }
    }

    if path != "" {
        opts.path = Some(path.trim_end().to_string());
    }

    opts
}
