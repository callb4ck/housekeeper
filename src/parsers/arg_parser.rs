use std::{env::args, process::exit};

#[derive(Default)]
pub struct ArgOptions {
    pub single_thread: bool,
    pub dry_run: bool,
    pub path: Option<String>,
}

fn help() {
    println!(
        "Use: housekeeper [OPTS] [PATH]
    -h,  --help:          Show this help message
    -d,  --dry-run:       Simulate an execution
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
            "-h" | "--help" => help(),
            "-d" | "--dry-run" => opts.dry_run = true,
            "-st" | "--single-thread" => opts.single_thread = true,
            _ => {
                if !arg.starts_with('-') {
                    path.push_str(arg.as_str());
                    path.push(' ');
                }
            }
        }
    }

    if !path.is_empty() {
        opts.path = Some(path.trim_end().to_string());
    }

    opts
}
