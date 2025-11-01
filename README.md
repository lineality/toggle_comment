# toggle_comment

A Rust crate to toggle comments and indent/unindent lines in files.
This is intended to be a module to be included in other projects, but
this can be used directly if only for testing.

# Six Principle Areas of Functionality
### Toggle Back and Forth
1. single line to toggle-comment (# or //)
2. rust doc strings stye to single line toggle-comment(///)
3. list of single lines to toggle-comment

### Line-Changes, So cannot "toggle" simplistically
4. quasi-toggles "block-comment" /*commment*/ or """comment"""
- text 'block' (/* comment */ or """comment""")

### Indent
5. indent/unindent a single line (+/- four spaces)
6. indent/unindent a range of lines (+/- four spaces)

## Toggle Single Line Comment
```rust
use toggle_comment_indent_module::toggle_basic_singleline_comment;

// Auto-detects `//` or `#` from file extension
toggle_basic_singleline_comment("./script.py", 5)?;   // → `// code`
toggle_basic_singleline_comment("./script.py", 3)?; // → `# code`
```

## Toggle Rust Docstring
```rust
use toggle_comment_indent_module::toggle_rust_docstring_singleline_comment;

// Use `///` instead of `//`
toggle_rust_docstring_singleline_comment("./script.py", 10)?;
```

## Toggle Block Comments
```rust
use toggle_comment_indent_module::toggle_block_comment;

// Automatically add/remove markers around lines 5-10
// Detects: /* */ for C/Rust, """ for Python
toggle_block_comment("./script.py", 5, 10)?;
```

## Batch Toggle Multiple Lines
```rust
use toggle_comment_indent_module::toggle_multiple_basic_comments;

// Toggle lines 5, 10, 15, 20 in one pass
let lines = [5, 10, 15, 20];
toggle_multiple_basic_comments("./script.py", &lines)?;
```

## Indent Single Line

Add 4 spaces to the start of a line:

```rust
use toggle_comment_indent_module::indent_line;

// Indent line 5 of a file
indent_line("./script.py", 5)?;
```

## Unindent Single Line

Remove up to 4 spaces from the start of a line:

```rust
use toggle_comment_indent_module::unindent_line;

// Unindent line 5 of a file
unindent_line("./script.py", 5)?;
```


## Indent Range

Add 4 spaces to the start of multiple lines (inclusive range):

```rust
use toggle_comment_indent_module::indent_range;

// Indent lines 5 through 15 (inclusive)
indent_range("./script.py", 5, 15)?;
```


## Unindent Range

Remove up to 4 spaces from the start of multiple lines (inclusive range):

```rust
use toggle_comment_indent_module::unindent_range;

// Unindent lines 5 through 15 (inclusive)
unindent_range("./script.py", 5, 15)?;
```


## Toggle Block Range Standard-Comment
```rust
use toggle_comment_indent_module::execute_range_toggle_basic;

execute_range_toggle_basic(file_path, start_line, end_line)
```

## ~Toggle Block Rust-Docstring
```rust
use toggle_comment_indent_module::execute_range_toggle_docstring;

execute_range_toggle_docstring(file_path, start_line, end_line)
```

## Supported Languages
This module needs the file to have a file extension.
```
| Language       | Extensions                | Comment     | Block   |
|----------------|---------------------------|-------------|---------|
| Rust           | `.rs`                     | `//`, `///` | `/* */` |
| C/C++          | `.c`, `.cpp`, `.h`, `.hpp`| `//`        | `/* */` |
| Python         | `.py`                     | `#`         | `"""`   |
| JavaScript     | `.js`, `.ts`              | `//`        | `/* */` |
| Shell          | `.sh`, `.bash`            | `#`         |         |
| TOML/YAML      | `.toml`, `.yaml`          | `#`         |         |
| Go, Java, Swift| `.go`, `.java`, `.swift`  | `//`        | `/* */` |
| Ruby, Perl, R  | `.rb`, `.pl`, `.r`        | `#`         |         |
```

## Example of Single Line Toggle:
```
Original:  "let x = 5;"
Toggle:    "// let x = 5;"   ← Added comment
Toggle:    "let x = 5;"      ← Removed comment
```

**Block mode:**
```
Input lines 0-2 (zero-index):
    line 1 (ide, 1-indexed)
    line 2 (ide, 1-indexed)
    line 3 (ide, 1-indexed)

Output (Add):
    /*
    line 1 (ide, 1-indexed)
    line 2 (ide, 1-indexed)
    line 3 (ide, 1-indexed)
    */
----------------
Input lines 0-4 (zero-index):
    /*
    line 2 (ide, 1-indexed)
    line 3 (ide, 1-indexed)
    line 4 (ide, 1-indexed)
    */

Output (Remove):
    line 1 (ide, 1-indexed)
    line 2 (ide, 1-indexed)
    line 3 (ide, 1-indexed)
```

## Safety Guarantees

- ✓ **Atomic**: Original file only modified on success
- ✓ **Backed up**: Auto-creates `backup_toggle_comment_{filename}`
- ✓ **Bounded**: Rejects lines > 1MB, batches ≤ 128 lines
- ✓ **No panics**: All errors returned as `Result`
- ✓ **Preserves**: Maintains line endings (LF/CRLF/none), whitespace, tabs
- ✓ **Stateless**: No dependencies on previous operations

## Scope
- Comment-flags at start of line only
- File extensions only, no attempted language-syntax analysis
- One-line comment pattern detection: `{n spaces}{flag}{1 space}`

## Errors

```rust
pub enum ToggleError {
    FileNotFound,
    NoExtension,
    UnsupportedExtension,
    LineNotFound { requested: usize, file_lines: usize },
    IoError(IoOperation),
    PathError,
    LineTooLong { line_number: usize, length: usize },
    InconsistentBlockMarkers,
    InvalidLineRange,
}
```

## Performance
```
| Operation    | Time       | Memory     |
|--------------|------------|------------|
| Single line  | O(n)       | Stack only |
| Batch (128)  | O(n log m) | Stack only |
| Block toggle | O(n)       | Stack only |
```
n = file lines, m = batch size

## Example: CLI Usage

```bash
# Toggle comment on line 5 of main.rs
cargo run -- ./script.py 5

# Exit codes:
# 0: success
# 2: file not found
# 3: no extension
# 4: unsupported extension
# 5: line not found
# 6: I/O error
# 7: path error
# 8: line too long
```

## Testing

```bash
# Run all tests including edge cases
cargo test --release

# Test coverage includes:
# ✓ Single-line add/remove
# ✓ Line ending variations (LF, CRLF, no newline)
# ✓ Indentation (spaces, tabs)
# ✓ Empty/whitespace-only lines
# ✓ Block comments (add/remove)
# ✓ Batch operations (sorted, duplicates, OOB)
# ✓ Error cases (not found, unsupported, etc.)
```


## No Dependencies

No 3rd party crates, only Rust standard library.
