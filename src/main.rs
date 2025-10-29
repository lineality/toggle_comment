//! Command-line interface for toggle_comment crate
//!
//! Usage: toggle_comment <file_path> <line_number>
//!
//! Example: toggle_comment ./src/main.rs 5

use std::env;
use std::process;
mod toggle_comment_module;
use toggle_comment_module::{ToggleError, toggle_comment};

/// Print usage information and exit
fn print_usage() {
    eprintln!("Usage: toggle_comment <file_path> <line_number>");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  file_path    - Path to source code file");
    eprintln!("  line_number  - Line number to toggle (zero-indexed)");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  toggle_comment ./src/main.rs 5");
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

    // Execute toggle_comment
    match toggle_comment(file_path, line_number) {
        Ok(()) => {
            println!("Successfully toggled comment on line {}", line_number);
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);

            // Return specific exit codes for different error types
            let exit_code = match e {
                ToggleError::FileNotFound(_) => 2,
                ToggleError::NoExtension(_) => 3,
                ToggleError::UnsupportedExtension(_) => 4,
                ToggleError::LineNotFound { .. } => 5,
                ToggleError::IoError { .. } => 6,
                ToggleError::PathError(_) => 7,
                ToggleError::LineTooLong { .. } => 8,
            };

            process::exit(exit_code);
        }
    }
}
