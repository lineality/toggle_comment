//! Command-line interface for toggle_basic_singleline_comment crate
//!
//! Usage: toggle_basic_singleline_comment <file_path> <line_number>
//!
//! Example: toggle_basic_singleline_comment ./src/main.rs 5

use std::env;
use std::process;
mod toggle_comment_module;
use toggle_comment_module::{ToggleError, toggle_basic_singleline_comment};

/// Print usage information and exit
fn print_usage() {
    eprintln!("Usage: toggle_basic_singleline_comment <file_path> <line_number>");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  file_path    - Path to source code file");
    eprintln!("  line_number  - Line number to toggle (zero-indexed)");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  toggle_basic_singleline_comment ./src/main.rs 5");
    eprintln!();
    eprintln!("Supported extensions:");
    eprintln!("  // : rs, c, cpp, js, ts, java, go, swift");
    eprintln!("  #  : py, sh, toml, yaml, rb, pl, r");
}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check argument count
    if args.len() != 3 {
        eprintln!("Error: Invalid number of arguments");
        print_usage();
        process::exit(1);
    }

    // Parse arguments
    let file_path = &args[1];
    let line_number = match args[2].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Line number must be a valid integer");
            print_usage();
            process::exit(1);
        }
    };

    // Execute toggle_basic_singleline_comment
    // Run toggle comment
    // return standard exit code:
    // zero is ok
    // error has number above zero
    match toggle_basic_singleline_comment(file_path, line_number) {
        Ok(()) => {
            println!("Successfully toggled comment on line {}", line_number);
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);

            // Return specific exit codes for different error types
            let exit_code = match e {
                ToggleError::FileNotFound => 2,
                ToggleError::NoExtension => 3,
                ToggleError::UnsupportedExtension => 4,
                ToggleError::LineNotFound { .. } => 5,
                ToggleError::IoError { .. } => 6,
                ToggleError::PathError => 7,
                ToggleError::LineTooLong { .. } => 8,
            };

            process::exit(exit_code);
        }
    }
}
