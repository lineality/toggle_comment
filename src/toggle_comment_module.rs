//! # toggle_comment_module.rs
//!
//! A simple, safe Rust crate to toggle comment flags on a specific line in source code files.
//!
//! ## Supported Languages
//! - Rust, C, C++, JavaScript, etc. (using `//` comment flag)
//! - Python, TOML, Shell scripts, etc. (using `#` comment flag)
//!
//! ## Supported Comment-Flag Modes
//! - single-line comments
//! - Rust doc string for single line '///'
//! - possible future feature: \n/*\n \n*/\n and \n"""\n \n"""\n
//!   added to new lines before and after the lines selected (not modifying within any line)
//!
//! ## Safety & Policy
//! - Never loads entire file into memory
//! - Pre-allocated fixed buffers only (no heap dynamic allocation during processing)
//! - Creates one static (not timestamped or versioned or unique) backup before any modifications
//!   Simple create/overwrite upon each operation
//! - Atomic file replacement (original only replaced on success)
//! - All errors returned as Result - no panics in production code
//! - Not swiss-army-knife functions: keep it simple, maintainable, communicable, plannable
//! - See more rules in comments below
//! - No event will result in a panic-crash, everything is handled.
//!
//! ## Usage
//! ```rust
//! use toggle_basic_singleline_comment::toggle_basic_singleline_comment;
//!
//! // Toggle comment on line 5 (zero-indexed) in main.rs
//! match toggle_basic_singleline_comment("/absolute/path/to/main.rs", 5) {
//!     Ok(()) => println!("Comment toggled successfully"),
//!     Err(e) => eprintln!("Error: {:?}", e),
//! }
//! ```

/*
TODO:
1. /// mode, separate function, not overloading existing code.
<NOT A COMBINED SWISS ARMY KNIFE FUNCTION!!!!!!!!!!!>
This should be able to operate in the same simple way as //
not conflicting.
if a line starts with {N spaces}{///}{1 space}, that is a match
otherwise not.
'//' does not conflict with this.
please be 100% clear IF you see a problem with this design.
can "// " collide with "/// " ? yes or no?

2. Is there a clean way to take in a list of lines,
so the file backup is not redundant for a 'set' of operations?
state-less no-heap is a challenge here but a good one.
what is realistic? what is practical?
Good-enough and maintainable is infinitly than a broken moonshot.
a list of 512 rows if memory-slim might be fine.


3. Is there a clean way to take in two line numbers
   and add comment-block lines before the first and after the 2nd?

   e.g.
   if those lines are not exact match: (not a pattern search: only exact match)
   /*\n
   */\n
   Then not-detected: toggle on

   if exact match, toggle-off

   language-extention based route is the same as before:
   .rs uses /**/, python uses """ """, etc.

   This should be nearly identical to adding or removing "// "
   the only differences are:
   - for rust there are two different patterns "\n/*\n" and "\n*/
\n", not a big problem
- we are looking not ahead for " // ", but looking at only 3 bytes (in the case of hash block),
     the middle of which is the symbol, before and after which are newlines.
   - there are two test conditions not one: both lines must be matched to be a match
   - there can be an edge case: if only one line matches: do nothing
   message: "one match"

   being an exact match, not forward pattern, should make it overall much simpler (if more steps)


4. Avoid heap for error messages and for all things.
   if this is big hastle (why do error-messages need heap???
   we can't set a terse error message???)
   then whatever, forget it,
   but lazy abuse of heap for recreational lazyiness
   turns important code into liablity toxic waste.
   Is heap used because that is THE best way
   the most secure, the most efficient?
   Or is heap used because "it's future dev's problem, let's party"
   Can we use  heap in debug mode only?
   The lack of clarity on this is not acceptable.
   Production software must not be a shrug punt.
   Is debug information being included in production builds?
   That is NOT supposed to happen.



Note: yes, if people have strange exotic code, this will not
try to work with that.
e.g.
```
////////////
```
will be ignored, that is by design.

the goal is to be able to turn-on, turn off, commments
reliably in this system, not to micro-manage the rest of the universe.


*/

/*
Example main function:
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
eprintln!(" // : rs, c, cpp, js, ts, java, go, swift");
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
*/

/*


# Rust rules:
-Always best practice.
-Always extensive doc strings.
-Always comments.
- Always cargo tests (where possible).
- Never remove documentation.
- Always clear, meaningful, unique names (e.g. variables, functions).
- Always absolute file paths.
- Always error handling.
- Never unsafe code.
- Never use unwrap.

- Load what is needed when it is needed: Do not ever load a whole file, rarely load a whole anything. increment and load only what is required pragmatically.

- Always defensive best practice:
- Always error handling: everything will fail at some point, if only because of cosmic-ray bit-flips (which are actually common), there must always be fail-safe error handling.

Safety, reliability, maintainability, fail-safe, communication-documentation, are the goals.

## No third party libraries (or very strictly avoid third party libraries where possible).

## Every part of code will eventually fail if only due to hardware failure, power supply failures, hard radiation bit flips, security attacks, etc. Every failure must be handled smoothly: let it fail and move on.

## Rule of Thumb, ideals not absolute rules: Follow NASA's 'Power of 10 rules' where possible and sensible (updated for 2025 and Rust):
1. no unsafe stuff:
- no recursion
- no goto
- no pointers
- no preprocessor

2. upper bound on all normal-loops, failsafe for all always-loops

3. Pre-allocate all memory (no dynamic memory allocation)

4. Clear function scope and Data Ownership: Part of having a function be 'focused' means knowing if the function is in scope. Functions should be neither swiss-army-knife functions that do too many things, nor scope-less micro-functions that may be doing something that should not be done. Many functions should have a narrow focus and a short length, but definition of actual-project scope functionality must be explicit. Replacing one long clear in-scope function with 50 scope-agnostic generic sub-functions with no clear way of telling if they are in scope or how they interact (e.g. hidden indirect recursion) is unsafe. Rust's ownership and borrowing rules focus on Data ownership and hidden dependencies, making it even less appropriate to scatter borrowing and ownership over a spray of microfunctions purely for the ideology of turning every operation into a microfunction just for the sake of doing so. (See more in rule 9.)

5. Defensive programming: debug-assert, test-assert, prod safely check & handle, not 'assert!' panic
For production-release code:
1. check and handle without panic/halt in production
2. return result (such as Result<T, E>) and smoothly handle errors (not halt-panic stopping the application): no assert!() outside of test-only code
3. test assert: use #[cfg(test)] assert!() to test production binaries (not in prod)
4. debug assert: use debug_assert to test debug builds/runs (not in prod)
5. use defensive programming with recovery of all issues at all times
- use cargo tests
- use debug_asserts
- do not leave assertions in production code.
- use no-panic error handling
- use Option
- use enums and structs
- check bounds
- check returns
- note: a test-flagged assert can test a production release build (whereas debug_assert cannot); cargo test --release
```
#[cfg(test)]
assert!(
```

e.g.
# "Assert & Catch-Handle" 3-part System

// template/example for check/assert format
//    =================================================
// // Debug-Assert, Test-Asset, Production-Catch-Handle
//    =================================================
// This is not included in production builds
// assert: only when running in a debug-build: will panic
debug_assert!(
    INFOBAR_MESSAGE_BUFFER_SIZE > 0,
    "Info bar buffer must have non-zero capacity"
);
// This is not included in production builds
// assert: only when running cargo test: will panic
#[cfg(test)]
assert!(
    INFOBAR_MESSAGE_BUFFER_SIZE > 0,
    "Info bar buffer must have non-zero capacity"
);
// Catch & Handle without panic in production
// This IS included in production to safe-catch
if !INFOBAR_MESSAGE_BUFFER_SIZE == 0 {
    // state.set_info_bar_message("Config error");
    return Err(LinesError::GeneralAssertionCatchViolation(
        "zero buffer size error".into(),
    ));
}


Avoid heap for error messages and for all things.
   Is heap used because that is THE best way
   the most secure, the most efficient, proper separate of debug testing vs. secure production code?
   Or is heap used because "it's future dev's problem, let's party"
   Can we use  heap in debug/test modes/builds only?
   The lack of clarity on this is not acceptable.
   Production software must not be a shrug punt.
   Is debug information being included in production builds?
   That is NOT supposed to happen.

This is central to the question about testing vs. a pedantic ban on conditional compilation; not putting full traceback insecurity into production code is not a different operational process logic tree for process operations.

Just like with the pedantic "a loops being bounded" rule, there is a fundamental exception: always-on loops must be the opposite.
With conditional compilations: code NEVER to EVER be in production-builds MUST be "conditionally" excluded. This is not an OS condition or a hardware condition. This is an 'unsafe-testing or not' condition.


6. ? Is this about ownership of variables?
- not sure how this translates into Rust's own-borrow rules...

7. manage return values:
- use null-void return values
- check non-void-null returns

8. Navigate debugging and testing on the one hand and not-dangerous conditional compilation on the other hand

9. Communicate:
- use doc strings, use comments,
- Document use-cases, edge-cases, and policies (These are project specific and cannot be telepathed from generic micro-function code. When a Mars satellite failed because one team used SI-metric units and another team did not, that problem could not have been detected by looking at, and auditing, any individual function in isolation without documentation. Breaking a process into innumerable undocumented micro-functions can make scope and policy impossible to track. To paraphrase Jack Welch: "The most dangerous thing in the world is a flawless operation that should never have been done in the first place.")

10. Use state-less operations when possible:
- a seemingly invisibly small increase in state often completely destroys projects
- expanding state destroys projects with unmaintainable over-reach

*/

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Maximum bytes to scan in a line when looking for comment pattern
/// This prevents unbounded operations on malformed files
const MAX_SCAN_BYTES: usize = 64;

/// Buffer size for file I/O operations - pre-allocated, stack-friendly size
const IO_BUFFER_SIZE: usize = 8192;

/// Maximum line length we'll process - safety bound
const MAX_LINE_LENGTH: usize = 1_000_000; // 64KB per line max

// /// Errors that can occur during comment toggling operations
// ///
// /// Each error variant provides specific context about what went wrong,
// /// allowing calling code to handle or report errors appropriately.
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum ToggleError {
//     /// The specified file was not found at the given path
//     FileNotFound(String),

//     /// File has no extension, cannot determine comment syntax
//     NoExtension(String),

//     /// File extension not recognized/supported for comment toggling
//     UnsupportedExtension(String),

//     /// The requested line index exceeds the file's line count
//     LineNotFound { requested: usize, file_lines: usize },

//     /// Generic I/O error with context about what operation failed
//     IoError { operation: String, details: String },

//     /// Path conversion or manipulation error
//     PathError(String),

//     /// Line exceeds maximum safe length
//     LineTooLong { line_number: usize, length: usize },
// }

/// Errors that can occur during comment toggling operations
///
/// All variants are Copy - no heap allocation, no string storage.
/// Caller provides context (file paths, etc.) they already have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleError {
    /// The specified file was not found
    FileNotFound,

    /// File has no extension
    NoExtension,

    /// File extension not recognized/supported
    UnsupportedExtension,

    /// The requested line index exceeds the file's line count
    LineNotFound { requested: usize, file_lines: usize },

    /// I/O operation failed
    IoError(IoOperation),

    /// Path conversion or manipulation error
    PathError,

    /// Line exceeds maximum safe length
    LineTooLong { line_number: usize, length: usize },
}

/// Specific I/O operations that can fail
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoOperation {
    /// Creating backup file
    Backup,
    /// Opening source file for reading
    Open,
    /// Creating temporary/destination file
    Create,
    /// Reading line from file
    Read,
    /// Writing line to file
    Write,
    /// Flushing write buffer
    Flush,
    /// Replacing original file with modified version
    Replace,
}

impl std::fmt::Display for ToggleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToggleError::FileNotFound => write!(f, "File not found"),
            ToggleError::NoExtension => write!(f, "No file extension"),
            ToggleError::UnsupportedExtension => write!(f, "Unsupported extension"),
            ToggleError::LineNotFound {
                requested,
                file_lines,
            } => {
                write!(
                    f,
                    "Line {} not found (file has {} lines)",
                    requested, file_lines
                )
            }
            ToggleError::IoError(op) => write!(f, "IO error: {:?}", op),
            ToggleError::PathError => write!(f, "Path error"),
            ToggleError::LineTooLong {
                line_number,
                length,
            } => {
                write!(f, "Line {} too long: {} bytes", line_number, length)
            }
        }
    }
}

impl std::error::Error for ToggleError {}

// impl std::fmt::Display for ToggleError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ToggleError::FileNotFound(path) => write!(f, "File not found: {}", path),
//             ToggleError::NoExtension(path) => write!(f, "No file extension: {}", path),
//             ToggleError::UnsupportedExtension(ext) => {
//                 write!(f, "Unsupported extension: {}", ext)
//             }
//             ToggleError::LineNotFound {
//                 requested,
//                 file_lines,
//             } => {
//                 write!(
//                     f,
//                     "Line {} not found (file has {} lines)",
//                     requested, file_lines
//                 )
//             }
//             ToggleError::IoError { operation, details } => {
//                 write!(f, "I/O error during {}: {}", operation, details)
//             }
//             ToggleError::PathError(msg) => write!(f, "Path error: {}", msg),
//             ToggleError::LineTooLong {
//                 line_number,
//                 length,
//             } => {
//                 write!(
//                     f,
//                     "Line {} exceeds maximum length: {} bytes",
//                     line_number, length
//                 )
//             }
//         }
//     }
// }

// impl std::error::Error for ToggleError {}

/// Comment flag type for different language syntaxes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommentFlag {
    /// Tripple Slash for Rust-Docstrings
    TripppleSlash,
    /// Double-slash comments (Rust, C, C++, JavaScript, etc.)
    DoubleSlash,
    /// Hash/pound comments (Python, Shell, TOML, etc.)
    Hash,
    // Python Type Doc Block
    QuoteBlockStart,
    // Rust Type Doc Block Start
    SlashBlockStart,
    // Rust Type Doc Block End
    SlashBlockEnd,
}

impl CommentFlag {
    /// Get the byte slice representation of the comment flag
    fn as_bytes(&self) -> &'static [u8] {
        match self {
            CommentFlag::TripppleSlash => b"///",
            CommentFlag::DoubleSlash => b"//",
            CommentFlag::Hash => b"#",
            CommentFlag::QuoteBlockStart => b"///",
            CommentFlag::SlashBlockStart => b"/*",
            CommentFlag::SlashBlockEnd => b"*/",
        }
    }

    /// Get the string representation of the comment flag
    fn as_str(&self) -> &'static str {
        match self {
            CommentFlag::TripppleSlash => "///",
            CommentFlag::DoubleSlash => "//",
            CommentFlag::Hash => "#",
            CommentFlag::QuoteBlockStart => "///",
            CommentFlag::SlashBlockStart => "/*",
            CommentFlag::SlashBlockEnd => "*/",
        }
    }
}

/// Determine comment flag based on file extension
///
/// # Arguments
/// * `extension` - File extension without the dot (e.g., "rs", "py")
///
/// # Returns
/// * `Some(CommentFlag)` if extension is supported
/// * `None` if extension is not recognized
///
/// # Supported Extensions
/// - `//` : rs, c, cpp, cc, cxx, h, hpp, js, ts, java, go, swift
/// - `#`  : py, sh, bash, toml, yaml, yml, rb, pl, r
fn determine_comment_flag(extension: &str) -> Option<CommentFlag> {
    match extension.to_lowercase().as_str() {
        // Double-slash languages
        "rs" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "js" | "ts" | "java" | "go" | "swift" => {
            Some(CommentFlag::DoubleSlash)
        }

        // Hash languages
        "py" | "sh" | "bash" | "toml" | "yaml" | "yml" | "rb" | "pl" | "r" => {
            Some(CommentFlag::Hash)
        }

        // Unknown extension
        _ => None,
    }
}

/// Analyze a line to determine if it has a comment flag that should be removed
///
/// Scans the first MAX_SCAN_BYTES of the line looking for pattern:
/// `{0+ spaces}{comment_flag}{1 space}`
///
/// # Arguments
/// * `line_bytes` - The line content as bytes (without newline)
/// * `flag` - The comment flag to search for
///
/// # Returns
/// * `Some(remove_count)` - Number of bytes to skip if pattern found (flag + space)
/// * `None` - If pattern not found in first MAX_SCAN_BYTES
///
/// # Safety
/// - Bounded scan (max MAX_SCAN_BYTES bytes)
/// - No allocations
/// - Handles malformed UTF-8 safely (works on bytes)
fn should_remove_comment(line_bytes: &[u8], flag: CommentFlag) -> Option<usize> {
    let flag_bytes = flag.as_bytes();
    let flag_len = flag_bytes.len();

    // Scan limit: don't go beyond line length or MAX_SCAN_BYTES
    let scan_limit = std::cmp::min(line_bytes.len(), MAX_SCAN_BYTES);

    // Track position in line
    let mut pos = 0;

    // Skip leading spaces - bounded loop with upper limit
    let mut spaces_skipped = 0;
    while pos < scan_limit && line_bytes[pos] == b' ' {
        pos += 1;
        spaces_skipped += 1;

        // Safety: prevent infinite loop on pathological input
        if spaces_skipped > MAX_SCAN_BYTES {
            return None;
        }
    }

    // Check if we have room for flag + space
    if pos + flag_len + 1 > scan_limit {
        return None;
    }

    // Check if next bytes match the flag
    if &line_bytes[pos..pos + flag_len] != flag_bytes {
        return None;
    }

    // Check if next byte after flag is a space
    if line_bytes[pos + flag_len] != b' ' {
        return None;
    }

    // Pattern found: return how many bytes to skip (flag + one space)
    Some(flag_len + 1)
}
/// Toggle Rust documentation comment (///) on a specific line
///
/// # Overview
/// Operates identically to `toggle_basic_singleline_comment` but uses
/// the Rust documentation comment marker `///` instead of determining
/// the comment type from file extension.
///
/// # Arguments
/// * `file_path` - Path to the source file
/// * `row_line_zeroindex` - Zero-indexed line number to toggle
///
/// # Returns
/// * `Ok(())` - Comment toggled successfully
/// * `Err(ToggleError)` - Specific error code
///
/// # Example
/// ```no_run
/// use toggle_basic_singleline_comment::toggle_rust_docstring_singleline_comment;
///
/// match toggle_rust_docstring_singleline_comment("./src/lib.rs", 0) {
///     Ok(()) => println!("Docstring toggled"),
///     Err(e) => eprintln!("Failed: {:?}", e),
/// }
/// ```
pub fn toggle_rust_docstring_singleline_comment(
    file_path: &str,
    row_line_zeroindex: usize,
) -> Result<(), ToggleError> {
    // Convert to absolute path
    let absolute_path = match Path::new(file_path).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Err(ToggleError::FileNotFound);
            }
            return Err(ToggleError::PathError);
        }
    };

    // Get filename for backup naming
    let filename = match absolute_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(ToggleError::PathError),
    };

    // Use TripleSlash flag (no extension lookup needed)
    let comment_flag = CommentFlag::TripppleSlash;

    // Create backup path in CWD
    let backup_filename = format!("backup_toggle_docstring_{}", filename);
    let backup_path = PathBuf::from(&backup_filename);

    // Create backup copy
    if let Err(_) = std::fs::copy(&absolute_path, &backup_path) {
        return Err(ToggleError::IoError(IoOperation::Backup));
    }

    // Create working temp file in CWD
    let temp_filename = format!("temp_toggle_docstring_{}_{}", std::process::id(), filename);
    let temp_path = PathBuf::from(&temp_filename);

    // Process file and toggle comment on target line
    let process_result =
        process_file_toggle(&absolute_path, &temp_path, row_line_zeroindex, comment_flag);

    // Handle result
    match process_result {
        Ok(()) => {
            // Success: replace original
            if let Err(_) = std::fs::copy(&temp_path, &absolute_path) {
                let _ = std::fs::remove_file(&temp_path);
                return Err(ToggleError::IoError(IoOperation::Replace));
            }

            // Clean up temp file
            if let Err(_) = std::fs::remove_file(&temp_path) {
                // Non-fatal: temp file left behind but operation succeeded
                #[cfg(debug_assertions)]
                eprintln!("Warning: Failed to clean up temp file");
            }

            Ok(())
        }
        Err(e) => {
            // Failed: clean up and return error
            let _ = std::fs::remove_file(&temp_path);
            Err(e)
        }
    }
}
// TODO, if possible
// note: do last-first, avoid frameshift problem
pub fn toggle_block_comment(
    file_path: &str,
    start_rowline_zeroindex: usize,
    end_rowline_zeroindex: usize,
) -> Result<(), ToggleError> {
    Ok(())
}

// // TODO
// state-less no-heap is a challenge here but a good one.
// what is realistic? what is practical?
// Good-enough and maintainable is infinitly than a broken moonshot.
// a list of 256 rows if memory-slim might be fine.
// if the use has to do batches, whatever.
// error handling, if the user puts in more than 256 items:
// do nothing, or print "too many"
// // not 'stateful' just iterate through
// pub fn set_toggle_basic_single_lines_comment(
//     file_path: &str,
//     list_row_lines_zeroindex: N vec<useize>, (pseudo syntax)
// ) -> Result<(), ToggleError> {
//     Ok(())
// }

/// Toggle comment on a specific line in a source code file
///
/// # Overview
/// This function safely toggles a comment flag on a single line without loading
/// the entire file into memory. It creates a backup before any modifications and
/// only replaces the original file on success.
///
/// Comment type is determined by file extension.
///
/// # Arguments
/// * `file_path` - Path to the source file
/// * `row_line_zeroindex` - Zero-indexed line number to toggle
///
/// # Returns
/// * `Ok(())` - Comment toggled successfully
/// * `Err(ToggleError)` - Specific error code
///
/// # Example
/// ```no_run
/// use toggle_basic_singleline_comment::toggle_basic_singleline_comment;
///
/// match toggle_basic_singleline_comment("./src/main.rs", 0) {
///     Ok(()) => println!("Toggled"),
///     Err(e) => eprintln!("Failed: {:?}", e),
/// }
/// ```
pub fn toggle_basic_singleline_comment(
    file_path: &str,
    row_line_zeroindex: usize,
) -> Result<(), ToggleError> {
    // Convert to absolute path
    let absolute_path = match Path::new(file_path).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Err(ToggleError::FileNotFound);
            }
            return Err(ToggleError::PathError);
        }
    };

    // Extract and validate file extension
    let extension = match absolute_path.extension() {
        Some(ext) => ext.to_string_lossy().to_string(),
        None => return Err(ToggleError::NoExtension),
    };

    // Determine comment flag from extension
    let comment_flag = match determine_comment_flag(&extension) {
        Some(flag) => flag,
        None => return Err(ToggleError::UnsupportedExtension),
    };

    // Get filename for backup naming
    let filename = match absolute_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(ToggleError::PathError),
    };

    // Create backup path in CWD
    let backup_filename = format!("backup_toggle_comment_{}", filename);
    let backup_path = PathBuf::from(&backup_filename);

    // Create backup copy of original file
    if let Err(_) = std::fs::copy(&absolute_path, &backup_path) {
        return Err(ToggleError::IoError(IoOperation::Backup));
    }

    // Create working temp file in CWD
    let temp_filename = format!("temp_toggle_{}_{}", std::process::id(), filename);
    let temp_path = PathBuf::from(&temp_filename);

    // Process file and toggle comment on target line
    let process_result =
        process_file_toggle(&absolute_path, &temp_path, row_line_zeroindex, comment_flag);

    // Handle processing result
    match process_result {
        Ok(()) => {
            // Success: replace original with temp file
            if let Err(_) = std::fs::copy(&temp_path, &absolute_path) {
                // Failed to replace - clean up and error
                let _ = std::fs::remove_file(&temp_path);
                return Err(ToggleError::IoError(IoOperation::Replace));
            }

            // Clean up temp file
            if let Err(_) = std::fs::remove_file(&temp_path) {
                // Non-fatal: temp file left behind but operation succeeded
                #[cfg(debug_assertions)]
                eprintln!("Warning: Failed to clean up temp file");
            }

            Ok(())
        }
        Err(e) => {
            // Failed: clean up temp file and return error
            let _ = std::fs::remove_file(&temp_path);
            Err(e)
        }
    }
}

// /// Toggle comment on a specific line in a source code file
// ///
// /// # Overview
// /// This function safely toggles a comment flag on a single line without loading
// /// the entire file into memory. It creates a backup before any modifications and
// /// only replaces the original file on success.
// ///
// /// # Algorithm
// /// 1. Validate file exists and has supported extension
// /// 2. Determine comment flag from extension
// /// 3. Create static backup in CWD: `backup_toggle_comment_{filename}`
// /// 4. Create temporary working copy in CWD
// /// 5. Process file line-by-line:
// ///    - Target line: toggle comment
// ///    - Other lines: copy unchanged
// /// 6. Replace original with working copy
// /// 7. Clean up temporary working file
// ///
// /// # Comment Toggle Rules
// /// - **Remove**: If line starts with `{spaces}{flag}{space}`, remove flag+space
// /// - **Add**: Otherwise, insert `{flag} ` at start of line (position 0)
// ///
// /// # Arguments
// /// * `file_path` - Path to the source file (will be converted to absolute path)
// /// * `row_line_zeroindex` - Zero-indexed line number to toggle (0 = first line)
// ///
// /// # Returns
// /// * `Ok(())` - Comment toggled successfully
// /// * `Err(ToggleError)` - Specific error describing what went wrong
// ///
// /// # Errors
// /// - `FileNotFound` - File doesn't exist
// /// - `NoExtension` - File has no extension
// /// - `UnsupportedExtension` - Extension not recognized
// /// - `LineNotFound` - Line index exceeds file length
// /// - `IoError` - File I/O operation failed
// /// - `PathError` - Path manipulation failed
// /// - `LineTooLong` - Line exceeds MAX_LINE_LENGTH safety limit
// ///
// /// # Safety Guarantees
// /// - Original file never modified until success verified
// /// - Backup created before any changes
// /// - All buffers pre-allocated with fixed sizes
// /// - Bounded loops with upper limits
// /// - No panics - all errors returned as Result
// ///
// /// # Example
// /// ```no_run
// /// use toggle_basic_singleline_comment::toggle_basic_singleline_comment;
// ///
// /// // Toggle comment on first line of main.rs
// /// match toggle_basic_singleline_comment("./src/main.rs", 0) {
// ///     Ok(()) => println!("Toggled successfully"),
// ///     Err(e) => eprintln!("Failed: {}", e),
// /// }
// /// ```
// pub fn toggle_basic_singleline_comment(
//     file_path: &str,
//     row_line_zeroindex: usize,
// ) -> Result<(), ToggleError> {
//     // Step 1: Convert to absolute path for safety and clarity
//     let absolute_path = match Path::new(file_path).canonicalize() {
//         Ok(p) => p,
//         Err(e) => {
//             if e.kind() == std::io::ErrorKind::NotFound {
//                 return Err(ToggleError::FileNotFound(file_path.to_string()));
//             }
//             return Err(ToggleError::PathError(format!(
//                 "Cannot canonicalize path: {}",
//                 e
//             )));
//         }
//     };

//     // Step 2: Extract and validate file extension
//     let extension = match absolute_path.extension() {
//         Some(ext) => ext.to_string_lossy().to_string(),
//         None => {
//             return Err(ToggleError::NoExtension(
//                 absolute_path.display().to_string(),
//             ));
//         }
//     };

//     // Step 3: Determine comment flag from extension
//     let comment_flag = match determine_comment_flag(&extension) {
//         Some(flag) => flag,
//         None => {
//             return Err(ToggleError::UnsupportedExtension(extension));
//         }
//     };

//     // Step 4: Get filename for backup naming
//     let filename = match absolute_path.file_name() {
//         Some(name) => name.to_string_lossy().to_string(),
//         None => {
//             return Err(ToggleError::PathError(
//                 "Cannot extract filename".to_string(),
//             ));
//         }
//     };

//     // Step 5: Create backup path in CWD (not source directory)
//     let backup_filename = format!("backup_toggle_comment_{}", filename);
//     let backup_path = PathBuf::from(&backup_filename);

//     // Step 6: Create backup copy of original file
//     if let Err(e) = std::fs::copy(&absolute_path, &backup_path) {
//         return Err(ToggleError::IoError {
//             operation: "create backup".to_string(),
//             details: format!("{}", e),
//         });
//     }

//     // Step 7: Create working temp file in CWD
//     let temp_filename = format!("temp_toggle_{}_{}", std::process::id(), filename);
//     let temp_path = PathBuf::from(&temp_filename);

//     // Step 8: Process file and toggle comment on target line
//     let process_result =
//         process_file_toggle(&absolute_path, &temp_path, row_line_zeroindex, comment_flag);

//     // Step 9: Handle processing result
//     match process_result {
//         Ok(()) => {
//             // Success: replace original with temp file
//             if let Err(e) = std::fs::copy(&temp_path, &absolute_path) {
//                 // Failed to replace - clean up and error
//                 let _ = std::fs::remove_file(&temp_path);
//                 return Err(ToggleError::IoError {
//                     operation: "replace original file".to_string(),
//                     details: format!("{}", e),
//                 });
//             }

//             // Step 10: Clean up temp file
//             if let Err(e) = std::fs::remove_file(&temp_path) {
//                 // Non-fatal: temp file left behind but operation succeeded
//                 debug_assert!(false, "Failed to clean up temp file: {}", e);
//             }

//             Ok(())
//         }
//         Err(e) => {
//             // Failed: clean up temp file and return error
//             let _ = std::fs::remove_file(&temp_path);
//             Err(e)
//         }
//     }
// }

// /// Process file line-by-line, toggling comment on target line
// ///
// /// # Arguments
// /// * `source_path` - Original file to read from
// /// * `dest_path` - Temporary file to write modified content to
// /// * `target_line` - Zero-indexed line number to toggle
// /// * `flag` - Comment flag to use
// ///
// /// # Returns
// /// * `Ok(())` - Processing succeeded, target line was found and toggled
// /// * `Err(ToggleError)` - Processing failed
// ///
// /// # Safety
// /// - Pre-allocated buffers only
// /// - Bounded line length checks
// /// - No dynamic allocation during loop
// fn process_file_toggle(
//     source_path: &Path,
//     dest_path: &Path,
//     target_line: usize,
//     flag: CommentFlag,
// ) -> Result<(), ToggleError> {
//     // Open source file for reading
//     let source_file = match File::open(source_path) {
//         Ok(f) => f,
//         Err(e) => {
//             return Err(ToggleError::IoError {
//                 operation: "open source file".to_string(),
//                 details: format!("{}", e),
//             });
//         }
//     };

//     // Create buffered reader with pre-allocated buffer
//     let mut reader = BufReader::with_capacity(IO_BUFFER_SIZE, source_file);

//     // Create destination file
//     let dest_file = match OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(dest_path)
//     {
//         Ok(f) => f,
//         Err(e) => {
//             return Err(ToggleError::IoError {
//                 operation: "create temp file".to_string(),
//                 details: format!("{}", e),
//             });
//         }
//     };

//     // Create buffered writer with pre-allocated buffer
//     let mut writer = BufWriter::with_capacity(IO_BUFFER_SIZE, dest_file);

//     // Pre-allocate line buffer - fixed size, reused for all lines
//     let mut line_buffer = Vec::with_capacity(MAX_LINE_LENGTH);

//     // Track current line number (zero-indexed)
//     let mut current_line: usize = 0;

//     // Track if we found the target line
//     let mut found_target = false;

//     // Line counter safety limit - prevent infinite loops
//     let line_limit = target_line.saturating_add(1000000); // Allow 1M lines past target

//     // Process file line by line
//     loop {
//         // Safety check: prevent unbounded loop
//         if current_line > line_limit {
//             return Err(ToggleError::IoError {
//                 operation: "process file".to_string(),
//                 details: "Line limit exceeded - possible infinite loop".to_string(),
//             });
//         }

//         // Clear buffer for reuse
//         line_buffer.clear();

//         // Read next line into pre-allocated buffer
//         let bytes_read = match reader.read_until(b'\n', &mut line_buffer) {
//             Ok(n) => n,
//             Err(e) => {
//                 return Err(ToggleError::IoError {
//                     operation: "read line".to_string(),
//                     details: format!("{}", e),
//                 });
//             }
//         };

//         // End of file reached
//         if bytes_read == 0 {
//             break;
//         }

//         // Safety: check line length
//         if line_buffer.len() > MAX_LINE_LENGTH {
//             return Err(ToggleError::LineTooLong {
//                 line_number: current_line,
//                 length: line_buffer.len(),
//             });
//         }

//         // Check if this is our target line
//         if current_line == target_line {
//             found_target = true;

//             // Toggle comment on this line
//             if let Err(e) = toggle_line(&mut writer, &line_buffer, flag) {
//                 return Err(e);
//             }
//         } else {
//             // Copy line unchanged
//             if let Err(e) = writer.write_all(&line_buffer) {
//                 return Err(ToggleError::IoError {
//                     operation: "write line".to_string(),
//                     details: format!("{}", e),
//                 });
//             }
//         }

//         current_line += 1;
//     }

//     // Flush writer to ensure all data written
//     if let Err(e) = writer.flush() {
//         return Err(ToggleError::IoError {
//             operation: "flush writer".to_string(),
//             details: format!("{}", e),
//         });
//     }

//     // Check if we found the target line
//     if !found_target {
//         return Err(ToggleError::LineNotFound {
//             requested: target_line,
//             file_lines: current_line,
//         });
//     }

//     Ok(())
// }

/// Process file line-by-line, toggling comment on target line
///
/// # Arguments
/// * `source_path` - Original file to read from
/// * `dest_path` - Temporary file to write modified content to
/// * `target_line` - Zero-indexed line number to toggle
/// * `flag` - Comment flag to use
///
/// # Returns
/// * `Ok(())` - Processing succeeded, target line was found and toggled
/// * `Err(ToggleError)` - Processing failed
///
/// # Safety
/// - Pre-allocated buffers only
/// - Bounded line length checks
/// - No dynamic allocation during loop
fn process_file_toggle(
    source_path: &Path,
    dest_path: &Path,
    target_line: usize,
    flag: CommentFlag,
) -> Result<(), ToggleError> {
    // Open source file for reading
    let source_file = match File::open(source_path) {
        Ok(f) => f,
        Err(_) => return Err(ToggleError::IoError(IoOperation::Open)),
    };

    // Create buffered reader with pre-allocated buffer
    let mut reader = BufReader::with_capacity(IO_BUFFER_SIZE, source_file);

    // Create destination file
    let dest_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dest_path)
    {
        Ok(f) => f,
        Err(_) => return Err(ToggleError::IoError(IoOperation::Create)),
    };

    // Create buffered writer with pre-allocated buffer
    let mut writer = BufWriter::with_capacity(IO_BUFFER_SIZE, dest_file);

    // Pre-allocate line buffer - fixed size, reused for all lines
    let mut line_buffer = Vec::with_capacity(MAX_LINE_LENGTH);

    // Track current line number (zero-indexed)
    let mut current_line: usize = 0;

    // Track if we found the target line
    let mut found_target = false;

    // Line counter safety limit - prevent infinite loops
    let line_limit = target_line.saturating_add(1000000);

    // Process file line by line
    loop {
        // Safety check: prevent unbounded loop
        if current_line > line_limit {
            return Err(ToggleError::IoError(IoOperation::Read));
        }

        // Clear buffer for reuse
        line_buffer.clear();

        // Read next line into pre-allocated buffer
        let bytes_read = match reader.read_until(b'\n', &mut line_buffer) {
            Ok(n) => n,
            Err(_) => return Err(ToggleError::IoError(IoOperation::Read)),
        };

        // End of file reached
        if bytes_read == 0 {
            break;
        }

        // Safety: check line length
        if line_buffer.len() > MAX_LINE_LENGTH {
            return Err(ToggleError::LineTooLong {
                line_number: current_line,
                length: line_buffer.len(),
            });
        }

        // Check if this is our target line
        if current_line == target_line {
            found_target = true;

            // Toggle comment on this line
            if let Err(e) = toggle_line(&mut writer, &line_buffer, flag) {
                return Err(e);
            }
        } else {
            // Copy line unchanged
            if let Err(_) = writer.write_all(&line_buffer) {
                return Err(ToggleError::IoError(IoOperation::Write));
            }
        }

        current_line += 1;
    }

    // Flush writer to ensure all data written
    if let Err(_) = writer.flush() {
        return Err(ToggleError::IoError(IoOperation::Flush));
    }

    // Check if we found the target line
    if !found_target {
        return Err(ToggleError::LineNotFound {
            requested: target_line,
            file_lines: current_line,
        });
    }

    Ok(())
}

// /// Toggle comment flag on a single line
// ///
// /// # Arguments
// /// * `writer` - Buffered writer to output modified line
// /// * `line_buffer` - Complete line including newline
// /// * `flag` - Comment flag to toggle
// ///
// /// # Returns
// /// * `Ok(())` - Line written successfully
// /// * `Err(ToggleError)` - Write failed
// ///
// /// # Logic
// /// - Extract line content (without trailing newline)
// /// - Check if comment should be removed
// /// - If yes: write line without flag+space
// /// - If no: write flag+space at start, then rest of line
// /// - Always preserve original newline
// fn toggle_line(
//     writer: &mut BufWriter<File>,
//     line_buffer: &[u8],
//     flag: CommentFlag,
// ) -> Result<(), ToggleError> {
//     // Separate line content from newline
//     let (content, newline) = if line_buffer.ends_with(b"\r\n") {
//         (&line_buffer[..line_buffer.len() - 2], &b"\r\n"[..])
//     } else if line_buffer.ends_with(b"\n") {
//         (&line_buffer[..line_buffer.len() - 1], &b"\n"[..])
//     } else {
//         // No newline (last line of file might not have one)
//         (line_buffer, &b""[..])
//     };

//     // Check if we should remove comment
//     if let Some(skip_count) = should_remove_comment(content, flag) {
//         // REMOVE mode: write content skipping flag+space
//         if let Err(e) = writer.write_all(&content[skip_count..]) {
//             return Err(ToggleError::IoError {
//                 operation: "write line (remove comment)".to_string(),
//                 details: format!("{}", e),
//             });
//         }
//     } else {
//         // ADD mode: write flag+space, then content
//         let flag_with_space = format!("{} ", flag.as_str());

//         if let Err(e) = writer.write_all(flag_with_space.as_bytes()) {
//             return Err(ToggleError::IoError {
//                 operation: "write comment flag".to_string(),
//                 details: format!("{}", e),
//             });
//         }

//         if let Err(e) = writer.write_all(content) {
//             return Err(ToggleError::IoError {
//                 operation: "write line (add comment)".to_string(),
//                 details: format!("{}", e),
//             });
//         }
//     }

//     // Write newline back (preserve original line ending)
//     if !newline.is_empty() {
//         if let Err(e) = writer.write_all(newline) {
//             return Err(ToggleError::IoError {
//                 operation: "write newline".to_string(),
//                 details: format!("{}", e),
//             });
//         }
//     }

//     Ok(())
// }

/// Toggle comment flag on a single line
///
/// # Arguments
/// * `writer` - Buffered writer to output modified line
/// * `line_buffer` - Complete line including newline
/// * `flag` - Comment flag to toggle
///
/// # Returns
/// * `Ok(())` - Line written successfully
/// * `Err(ToggleError)` - Write failed
///
/// # Logic
/// - Extract line content (without trailing newline)
/// - Check if comment should be removed
/// - If yes: write line without flag+space
/// - If no: write flag+space at start, then rest of line
/// - Always preserve original newline
fn toggle_line(
    writer: &mut BufWriter<File>,
    line_buffer: &[u8],
    flag: CommentFlag,
) -> Result<(), ToggleError> {
    // Separate line content from newline
    let (content, newline) = if line_buffer.ends_with(b"\r\n") {
        (&line_buffer[..line_buffer.len() - 2], &b"\r\n"[..])
    } else if line_buffer.ends_with(b"\n") {
        (&line_buffer[..line_buffer.len() - 1], &b"\n"[..])
    } else {
        // No newline (last line of file might not have one)
        (line_buffer, &b""[..])
    };

    // Check if we should remove comment
    if let Some(skip_count) = should_remove_comment(content, flag) {
        // REMOVE mode: write content skipping flag+space
        if let Err(_) = writer.write_all(&content[skip_count..]) {
            return Err(ToggleError::IoError(IoOperation::Write));
        }
    } else {
        // ADD mode: write flag+space, then content
        let flag_with_space = format!("{} ", flag.as_str());

        if let Err(_) = writer.write_all(flag_with_space.as_bytes()) {
            return Err(ToggleError::IoError(IoOperation::Write));
        }

        if let Err(_) = writer.write_all(content) {
            return Err(ToggleError::IoError(IoOperation::Write));
        }
    }

    // Write newline back (preserve original line ending)
    if !newline.is_empty() {
        if let Err(_) = writer.write_all(newline) {
            return Err(ToggleError::IoError(IoOperation::Write));
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod toggle_comment_tests {
    use super::*;
    use std::io::Write;

    /// Helper: create a temporary test file with given content
    fn create_test_file(filename: &str, content: &str) -> PathBuf {
        let path = PathBuf::from(filename);
        let mut file = File::create(&path).expect("Failed to create test file");
        file.write_all(content.as_bytes())
            .expect("Failed to write test file");
        path
    }

    /// Helper: read file content as string
    fn read_file_content(path: &Path) -> String {
        std::fs::read_to_string(path).expect("Failed to read file")
    }

    /// Helper: cleanup test files
    fn cleanup_files(paths: &[&Path]) {
        for path in paths {
            let _ = std::fs::remove_file(path);
        }
    }

    #[test]
    fn test_determine_comment_flag() {
        // Double-slash languages
        assert_eq!(determine_comment_flag("rs"), Some(CommentFlag::DoubleSlash));
        assert_eq!(
            determine_comment_flag("cpp"),
            Some(CommentFlag::DoubleSlash)
        );
        assert_eq!(determine_comment_flag("js"), Some(CommentFlag::DoubleSlash));

        // Hash languages
        assert_eq!(determine_comment_flag("py"), Some(CommentFlag::Hash));
        assert_eq!(determine_comment_flag("toml"), Some(CommentFlag::Hash));
        assert_eq!(determine_comment_flag("sh"), Some(CommentFlag::Hash));

        // Unknown
        assert_eq!(determine_comment_flag("txt"), None);
        assert_eq!(determine_comment_flag("unknown"), None);
    }

    #[test]
    fn test_should_remove_comment_rust() {
        let flag = CommentFlag::DoubleSlash;

        // Should remove: "// code"
        assert_eq!(should_remove_comment(b"// code", flag), Some(3));

        // Should remove: "  // code"
        assert_eq!(should_remove_comment(b"  // code", flag), Some(3));

        // Should NOT remove: "//code" (no space after flag)
        assert_eq!(should_remove_comment(b"//code", flag), None);

        // Should NOT remove: "code // comment"
        assert_eq!(should_remove_comment(b"code // comment", flag), None);

        // Should NOT remove: empty line
        assert_eq!(should_remove_comment(b"", flag), None);

        // Should NOT remove: only spaces
        assert_eq!(should_remove_comment(b"    ", flag), None);
    }

    #[test]
    fn test_should_remove_comment_python() {
        let flag = CommentFlag::Hash;

        // Should remove: "# code"
        assert_eq!(should_remove_comment(b"# code", flag), Some(2));

        // Should remove: "  # code"
        assert_eq!(should_remove_comment(b"  # code", flag), Some(2));

        // Should NOT remove: "#code" (no space after flag)
        assert_eq!(should_remove_comment(b"#code", flag), None);

        // Should NOT remove: "code # comment"
        assert_eq!(should_remove_comment(b"code # comment", flag), None);
    }

    #[test]
    fn test_toggle_comment_add_rust() {
        let test_file = create_test_file("test_add.rs", "fn main() {}\n");

        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 0);

        #[cfg(test)]
        assert!(result.is_ok());

        let content = read_file_content(&test_file);

        #[cfg(test)]
        assert_eq!(content, "// fn main() {}\n");

        // Cleanup
        cleanup_files(&[
            &test_file,
            &PathBuf::from("backup_toggle_comment_test_add.rs"),
        ]);
    }

    #[test]
    fn test_toggle_comment_remove_rust() {
        let test_file = create_test_file("test_remove.rs", "// fn main() {}\n");

        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 0);

        #[cfg(test)]
        assert!(result.is_ok());

        let content = read_file_content(&test_file);

        #[cfg(test)]
        assert_eq!(content, "fn main() {}\n");

        // Cleanup
        cleanup_files(&[
            &test_file,
            &PathBuf::from("backup_toggle_comment_test_remove.rs"),
        ]);
    }

    #[test]
    fn test_toggle_comment_add_python() {
        let test_file = create_test_file("test_add.py", "print('hello')\n");

        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 0);

        #[cfg(test)]
        assert!(result.is_ok());

        let content = read_file_content(&test_file);

        #[cfg(test)]
        assert_eq!(content, "# print('hello')\n");

        // Cleanup
        cleanup_files(&[
            &test_file,
            &PathBuf::from("backup_toggle_comment_test_add.py"),
        ]);
    }

    #[test]
    fn test_toggle_comment_remove_python() {
        let test_file = create_test_file("test_remove.py", "# print('hello')\n");

        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 0);

        #[cfg(test)]
        assert!(result.is_ok());

        let content = read_file_content(&test_file);

        #[cfg(test)]
        assert_eq!(content, "print('hello')\n");

        // Cleanup
        cleanup_files(&[
            &test_file,
            &PathBuf::from("backup_toggle_comment_test_remove.py"),
        ]);
    }

    #[test]
    fn test_toggle_comment_line_not_found() {
        let test_file = create_test_file("test_notfound.rs", "fn main() {}\n");

        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 10);

        #[cfg(test)]
        assert!(matches!(result, Err(ToggleError::LineNotFound { .. })));

        // Cleanup
        cleanup_files(&[
            &test_file,
            &PathBuf::from("backup_toggle_comment_test_notfound.rs"),
        ]);
    }

    #[test]
    fn test_toggle_comment_no_extension() {
        let test_file = create_test_file("test_noext", "some content\n");

        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 0);

        #[cfg(test)]
        assert!(matches!(result, Err(ToggleError::NoExtension)));

        // Cleanup
        cleanup_files(&[&test_file]);
    }

    #[test]
    fn test_toggle_comment_unsupported_extension() {
        let test_file = create_test_file("test.txt", "some content\n");

        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 0);

        #[cfg(test)]
        assert!(matches!(result, Err(ToggleError::UnsupportedExtension)));

        // Cleanup
        cleanup_files(&[&test_file]);
    }

    #[test]
    fn test_toggle_preserves_other_lines() {
        let content = "line 0\nline 1\nline 2\n";
        let test_file = create_test_file("test_preserve.rs", content);

        // Toggle line 1
        let result = toggle_basic_singleline_comment(test_file.to_str().unwrap(), 1);

        #[cfg(test)]
        assert!(result.is_ok());

        let new_content = read_file_content(&test_file);

        #[cfg(test)]
        assert_eq!(new_content, "line 0\n// line 1\nline 2\n");

        // Cleanup
        cleanup_files(&[
            &test_file,
            &PathBuf::from("backup_toggle_comment_test_preserve.rs"),
        ]);
    }
}
