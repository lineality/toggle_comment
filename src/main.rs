//! # main.rs
//!
//! Command-line interface for toggle_basic_singleline_comment crate
//!
//! # Usage Modes
//!
//! ## Basic single-line toggle (auto-detect comment type from extension)
//! ```text
//! toggle_comment <file_path> <line_number>
//! ```
//!
//! ## Rust docstring single-line toggle (///)
//! ```text
//! toggle_comment --rust-doc-string <file_path> <line_number>
//! ```
//!
//! ## Block comment toggle (insert/remove /* */ or """ """)
//! ```text
//! toggle_comment --block <file_path> <start_line> <end_line>
//! ```
//!
//! ## Batch toggle - basic comments
//! ```text
//! toggle_comment --list-basic <file_path> <line1> <line2> ... <lineN>
//! ```
//!
//! ## Batch toggle - rust docstrings
//! ```text
//! toggle_comment --list-docstring <file_path> <line1> <line2> ... <lineN>
//! ```

use std::env;
use std::process;
mod toggle_comment_module;
use toggle_comment_module::{
    IndentError, ToggleError, indent_line, toggle_basic_singleline_comment, toggle_block_comment,
    toggle_multiple_basic_comments, toggle_multiple_singline_docstrings,
    toggle_rust_docstring_singleline_comment, unindent_line,
};

/// Maximum number of lines that can be toggled in batch mode
/// Prevents unbounded memory usage while still being practical
const MAX_BATCH_LINES: usize = 512;

/// Print comprehensive usage information and exit
fn print_usage() {
    eprintln!("toggle_comment - Toggle comments in source code files");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  toggle_comment <file_path> <line_number>");
    eprintln!("  toggle_comment --rust-doc-string <file_path> <line_number>");
    eprintln!("  toggle_comment --block <file_path> <start_line> <end_line>");
    eprintln!("  toggle_comment --list-basic <file_path> <line1> <line2> ...");
    eprintln!("  toggle_comment --list-docstring <file_path> <line1> <line2> ...");
    eprintln!("  toggle_comment --indent <file_path> <line_number>");
    eprintln!("  toggle_comment --unindent <file_path> <line_number>");
    eprintln!();

    eprintln!("MODES:");
    eprintln!("  Basic mode:");
    eprintln!("    Auto-detects comment type from file extension");
    eprintln!("    Toggles // or # on a single line");
    eprintln!();
    eprintln!("  --rust-doc-string:");
    eprintln!("    Toggles Rust documentation comment (///) on a single line");
    eprintln!();
    eprintln!("  --block:");
    eprintln!("    Toggles block comments around a range of lines");
    eprintln!("    Inserts /* before start_line and */ after end_line (or removes them)");
    eprintln!("    For Python: uses \"\"\" instead");
    eprintln!();
    eprintln!("  --list-basic:");
    eprintln!("    Toggle basic comments on multiple lines in one operation");
    eprintln!("    Maximum {} lines per batch", MAX_BATCH_LINES);
    eprintln!();
    eprintln!("  --list-docstring:");
    eprintln!("    Toggle /// comments on multiple lines in one operation");
    eprintln!("    Maximum {} lines per batch", MAX_BATCH_LINES);
    eprintln!();
    eprintln!("  --indent:");
    eprintln!("    Add 4 spaces to the start of a line");
    eprintln!();
    eprintln!("  --unindent:");
    eprintln!("    Remove up to 4 spaces from the start of a line");
    eprintln!();

    eprintln!("ARGUMENTS:");
    eprintln!("  file_path    - Path to source code file");
    eprintln!("  line_number  - Line number to toggle (zero-indexed)");
    eprintln!("  start_line   - First line of block (zero-indexed)");
    eprintln!("  end_line     - Last line of block (zero-indexed)");
    eprintln!();

    eprintln!("EXAMPLES:");
    eprintln!("  toggle_comment hello_world.py 5");
    eprintln!("  toggle_comment --rust-doc-string hello_world.py 10");
    eprintln!("  toggle_comment --block hello_world.rs 5 15");
    eprintln!("  toggle_comment --list-basic hello_world.py 1 10 12");
    eprintln!("  toggle_comment --list-docstring hello_world.toml 1 2 3");
    eprintln!("  toggle_comment --indent hello_world.py 10");
    eprintln!("  toggle_comment --unindent hello_world.py 10");
    eprintln!("  toggle_comment --indent-range hello_world.py 10 12");
    eprintln!("  toggle_comment --unindent-range hello_world.py 10 12");
    eprintln!();

    eprintln!("SUPPORTED EXTENSIONS:");
    eprintln!("  //  : rs, c, cpp, js, ts, java, go, swift");
    eprintln!("  #   : py, sh, toml, yaml, rb, pl, r");
    eprintln!();

    eprintln!("EXIT CODES:");
    eprintln!("  0 - Success");
    eprintln!("  1 - Invalid arguments");
    eprintln!("  2 - File not found");
    eprintln!("  3 - No extension");
    eprintln!("  4 - Unsupported extension");
    eprintln!("  5 - Line not found");
    eprintln!("  6 - I/O error");
    eprintln!("  7 - Path error");
    eprintln!("  8 - Line too long");
}

/// Parse a line number argument, returning error on invalid input
///
/// # Arguments
/// * `arg` - String slice to parse
/// * `arg_name` - Name of argument for error messages
///
/// # Returns
/// * `Ok(usize)` - Successfully parsed line number
/// * `Err(())` - Parse failed (error already printed to stderr)
fn parse_line_number(arg: &str, arg_name: &str) -> Result<usize, ()> {
    match arg.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(_) => {
            eprintln!("Error: {} must be a valid integer", arg_name);
            eprintln!();
            Err(())
        }
    }
}

/// Parse multiple line number arguments into a fixed-size array
///
/// # Arguments
/// * `args` - Slice of string arguments to parse
///
/// # Returns
/// * `Ok((count, array))` - Successfully parsed line numbers
/// * `Err(())` - Parse failed or too many lines
///
/// # Safety
/// - Bounded to MAX_BATCH_LINES (512)
/// - Pre-allocated fixed array on stack
fn parse_line_list(args: &[String]) -> Result<(usize, [usize; MAX_BATCH_LINES]), ()> {
    // Check bounds
    if args.is_empty() {
        eprintln!("Error: No line numbers provided");
        return Err(());
    }

    if args.len() > MAX_BATCH_LINES {
        eprintln!("Error: Too many lines (max {})", MAX_BATCH_LINES);
        return Err(());
    }

    // Pre-allocate fixed array
    let mut line_array: [usize; MAX_BATCH_LINES] = [0; MAX_BATCH_LINES];
    let count = args.len();

    // Parse each line number
    for (i, arg) in args.iter().enumerate() {
        match arg.parse::<usize>() {
            Ok(n) => line_array[i] = n,
            Err(_) => {
                eprintln!("Error: Invalid line number: {}", arg);
                return Err(());
            }
        }
    }

    Ok((count, line_array))
}

// /// Convert ToggleError to exit code
// ///
// /// # Arguments
// /// * `error` - The error to convert
// ///
// /// # Returns
// /// * Exit code (2-8)
// fn error_to_exit_code(error: ToggleError) -> i32 {
//     match error {
//         ToggleError::FileNotFound => 2,
//         ToggleError::NoExtension => 3,
//         ToggleError::UnsupportedExtension => 4,
//         ToggleError::LineNotFound { .. } => 5,
//         ToggleError::IoError(_) => 6,
//         ToggleError::PathError => 7,
//         ToggleError::LineTooLong { .. } => 8,
//         ToggleError::InconsistentBlockMarkers => 9,
//         ToggleError::InvalidLineRange => 10,
//     }
// }
/// Convert ToggleError to exit code
///
/// # Arguments
/// * `error` - The error to convert
///
/// # Returns
/// * Exit code (2-10)
fn error_to_exit_code(error: ToggleError) -> i32 {
    match error {
        ToggleError::FileNotFound => 2,
        ToggleError::NoExtension => 3,
        ToggleError::UnsupportedExtension => 4,
        ToggleError::LineNotFound { .. } => 5,
        ToggleError::IoError(_) => 6,
        ToggleError::PathError => 7,
        ToggleError::LineTooLong { .. } => 8,
        ToggleError::InconsistentBlockMarkers => 9,
        ToggleError::InvalidLineRange => 10,
    }
}

/// Convert IndentError to exit code
///
/// # Arguments
/// * `error` - The error to convert
///
/// # Returns
/// * Exit code (2-10, same mapping as ToggleError where applicable)
fn indent_error_to_exit_code(error: IndentError) -> i32 {
    match error {
        IndentError::FileNotFound => 2,
        IndentError::LineNotFound { .. } => 5,
        IndentError::IoError(_) => 6,
        IndentError::PathError => 7,
        IndentError::LineTooLong { .. } => 8,
        IndentError::InvalidLineRange => 10,
    }
}

/// Execute indent on a single line
fn execute_indent(file_path: &str, line_number: usize) -> i32 {
    match indent_line(file_path, line_number) {
        Ok(()) => {
            println!("Successfully indented line {}", line_number);
            0
        }
        Err(e) => {
            eprintln!("Error indenting {}: {}", file_path, e);
            indent_error_to_exit_code(e)
        }
    }
}

/// Execute unindent on a single line
fn execute_unindent(file_path: &str, line_number: usize) -> i32 {
    match unindent_line(file_path, line_number) {
        Ok(()) => {
            println!("Successfully unindented line {}", line_number);
            0
        }
        Err(e) => {
            eprintln!("Error unindenting {}: {}", file_path, e);
            indent_error_to_exit_code(e)
        }
    }
}

/// Execute basic single-line comment toggle
fn execute_basic_toggle(file_path: &str, line_number: usize) -> i32 {
    match toggle_basic_singleline_comment(file_path, line_number) {
        Ok(()) => {
            println!("Successfully toggled comment on line {}", line_number);
            0
        }
        Err(e) => {
            eprintln!("Error toggling {}: {}", file_path, e);
            error_to_exit_code(e)
        }
    }
}

/// Execute Rust docstring single-line comment toggle
fn execute_docstring_toggle(file_path: &str, line_number: usize) -> i32 {
    match toggle_rust_docstring_singleline_comment(file_path, line_number) {
        Ok(()) => {
            println!("Successfully toggled docstring on line {}", line_number);
            0
        }
        Err(e) => {
            eprintln!("Error toggling docstring {}: {}", file_path, e);
            error_to_exit_code(e)
        }
    }
}

/// Execute block comment toggle
fn execute_block_toggle(file_path: &str, start_line: usize, end_line: usize) -> i32 {
    match toggle_block_comment(file_path, start_line, end_line) {
        Ok(()) => {
            println!(
                "Successfully toggled block comment (lines {}-{})",
                start_line, end_line
            );
            0
        }
        Err(e) => {
            eprintln!("Error toggling block {}: {}", file_path, e);
            error_to_exit_code(e)
        }
    }
}

// /// Execute batch toggle - basic comments (NOT IMPLEMENTED YET)
// fn execute_batch_toggle_standard(
//     file_path: &str,
//     _count: usize,
//     _lines: &[usize; MAX_BATCH_LINES],
// ) -> i32 {
//     eprintln!("Error: Batch toggle not yet implemented");
//     eprintln!("File: {}", file_path);
//     1
// }

// /// Execute batch toggle - docstrings (NOT IMPLEMENTED YET)
// fn execute_batch_toggle_docstring(
//     file_path: &str,
//     _count: usize,
//     _lines: &[usize; MAX_BATCH_LINES],
// ) -> i32 {
//     eprintln!("Error: Batch docstring toggle not yet implemented");
//     eprintln!("File: {}", file_path);
//     1
// }

/// Execute batch toggle - basic comments
fn execute_batch_toggle_standard(
    file_path: &str,
    count: usize,
    lines: &[usize; MAX_BATCH_LINES],
) -> i32 {
    // Use only the valid portion of array
    let line_slice = &lines[..count];

    match toggle_multiple_basic_comments(file_path, line_slice) {
        Ok(()) => {
            println!("Successfully toggled {} lines", count);
            0
        }
        Err(e) => {
            eprintln!("Error batch toggling {}: {}", file_path, e);
            error_to_exit_code(e)
        }
    }
}

/// Execute batch toggle - docstrings
fn execute_batch_toggle_docstring(
    file_path: &str,
    count: usize,
    lines: &[usize; MAX_BATCH_LINES],
) -> i32 {
    // Use only the valid portion of array
    let line_slice = &lines[..count];

    match toggle_multiple_singline_docstrings(file_path, line_slice) {
        Ok(()) => {
            println!("Successfully toggled {} docstrings", count);
            0
        }
        Err(e) => {
            eprintln!("Error batch toggling docstrings {}: {}", file_path, e);
            error_to_exit_code(e)
        }
    }
}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Minimum: program name + at least 2 args
    if args.len() < 3 {
        eprintln!("Error: Invalid number of arguments");
        eprintln!();
        print_usage();
        process::exit(1);
    }

    // Determine mode based on first argument
    let exit_code = if args[1].starts_with("--") {
        // Flag-based mode
        let flag = &args[1];

        match flag.as_str() {
            "--rust-doc-string" => {
                // Expect: --rust-doc-string <file> <line>
                if args.len() != 4 {
                    eprintln!("Error: --rust-doc-string requires <file_path> <line_number>");
                    eprintln!();
                    print_usage();
                    process::exit(1);
                }

                let file_path = &args[2];
                let line_number = match parse_line_number(&args[3], "line_number") {
                    Ok(n) => n,
                    Err(_) => {
                        print_usage();
                        process::exit(1);
                    }
                };

                execute_docstring_toggle(file_path, line_number)
            }

            "--block" => {
                // Expect: --block <file> <start_line> <end_line>
                if args.len() != 5 {
                    eprintln!("Error: --block requires <file_path> <start_line> <end_line>");
                    eprintln!();
                    print_usage();
                    process::exit(1);
                }

                let file_path = &args[2];
                let start_line = match parse_line_number(&args[3], "start_line") {
                    Ok(n) => n,
                    Err(_) => {
                        print_usage();
                        process::exit(1);
                    }
                };
                let end_line = match parse_line_number(&args[4], "end_line") {
                    Ok(n) => n,
                    Err(_) => {
                        print_usage();
                        process::exit(1);
                    }
                };

                // Validate line order
                if start_line >= end_line {
                    eprintln!("Error: start_line must be less than end_line");
                    process::exit(1);
                }

                execute_block_toggle(file_path, start_line, end_line)
            }

            "--list-basic" => {
                // Expect: --list-basic <file> <line1> <line2> ...
                if args.len() < 4 {
                    eprintln!("Error: --list-basic requires <file_path> <line1> [line2] ...");
                    eprintln!();
                    print_usage();
                    process::exit(1);
                }

                let file_path = &args[2];
                let line_args = &args[3..];

                let (count, line_array) = match parse_line_list(line_args) {
                    Ok(result) => result,
                    Err(_) => {
                        print_usage();
                        process::exit(1);
                    }
                };

                execute_batch_toggle_standard(file_path, count, &line_array)
            }

            "--list-docstring" => {
                // Expect: --list-docstring <file> <line1> <line2> ...
                if args.len() < 4 {
                    eprintln!("Error: --list-docstring requires <file_path> <line1> [line2] ...");
                    eprintln!();
                    print_usage();
                    process::exit(1);
                }

                let file_path = &args[2];
                let line_args = &args[3..];

                let (count, line_array) = match parse_line_list(line_args) {
                    Ok(result) => result,
                    Err(_) => {
                        print_usage();
                        process::exit(1);
                    }
                };

                execute_batch_toggle_docstring(file_path, count, &line_array)
            }
            "--indent" => {
                // Expect: --indent <file> <line>
                if args.len() != 4 {
                    eprintln!("Error: --indent requires <file_path> <line_number>");
                    eprintln!();
                    print_usage();
                    process::exit(1);
                }

                let file_path = &args[2];
                let line_number = match parse_line_number(&args[3], "line_number") {
                    Ok(n) => n,
                    Err(_) => {
                        print_usage();
                        process::exit(1);
                    }
                };

                execute_indent(file_path, line_number)
            }

            "--unindent" => {
                // Expect: --unindent <file> <line>
                if args.len() != 4 {
                    eprintln!("Error: --unindent requires <file_path> <line_number>");
                    eprintln!();
                    print_usage();
                    process::exit(1);
                }

                let file_path = &args[2];
                let line_number = match parse_line_number(&args[3], "line_number") {
                    Ok(n) => n,
                    Err(_) => {
                        print_usage();
                        process::exit(1);
                    }
                };

                execute_unindent(file_path, line_number)
            }
            // "--indent-range" => {
            //     // Expect: --block <file> <start_line> <end_line>
            //     if args.len() != 5 {
            //         eprintln!("Error: --block requires <file_path> <start_line> <end_line>");
            //         eprintln!();
            //         print_usage();
            //         process::exit(1);
            //     }

            //     let file_path = &args[2];
            //     let start_line = match parse_line_number(&args[3], "start_line") {
            //         Ok(n) => n,
            //         Err(_) => {
            //             print_usage();
            //             process::exit(1);
            //         }
            //     };
            //     let end_line = match parse_line_number(&args[4], "end_line") {
            //         Ok(n) => n,
            //         Err(_) => {
            //             print_usage();
            //             process::exit(1);
            //         }
            //     };

            //     // Validate line order
            //     if start_line >= end_line {
            //         eprintln!("Error: start_line must be less than end_line");
            //         process::exit(1);
            //     }

            //     execute_block_toggle(file_path, start_line, end_line)
            // }
            _ => {
                eprintln!("Error: Unknown flag: {}", flag);
                eprintln!();
                print_usage();
                process::exit(1);
            }
        }
    } else {
        // Basic mode: <file> <line>
        if args.len() != 3 {
            eprintln!("Error: Basic mode requires <file_path> <line_number>");
            eprintln!();
            print_usage();
            process::exit(1);
        }

        let file_path = &args[1];
        let line_number = match parse_line_number(&args[2], "line_number") {
            Ok(n) => n,
            Err(_) => {
                print_usage();
                process::exit(1);
            }
        };

        execute_basic_toggle(file_path, line_number)
    };

    // Exit with appropriate code
    process::exit(exit_code);
}
