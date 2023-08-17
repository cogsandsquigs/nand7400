mod tests;

use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Mutex;

// Regex for capturing multiple spaces in a row.
static MULTISPACE: Lazy<Mutex<Regex>> = Lazy::new(|| Mutex::new(Regex::new(r"\s+").unwrap()));

/// The main formatting structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Formatter {}

impl Formatter {
    /// Creates a new formatter.
    pub fn new() -> Formatter {
        Formatter {}
    }

    /// Formats the given assembly code. It does not check for errors, but instead properly indents and formats the code.
    pub fn format(&self, code: &str) -> String {
        let lines = code
            .lines()
            .map(str::trim)
            .map(|line| {
                MULTISPACE
                    .lock()
                    .expect("A mutex was poisoned!")
                    .replace_all(line, " ")
            })
            .collect::<Vec<_>>();

        let mut seen_label = false; // Indent everything after a label (except labels, ofc).
        let mut newline_count = 0; // Only allow at most 1 newline in a row.

        let mut formatted = String::new();

        for line in lines {
            if line.is_empty() {
                if newline_count == 0 {
                    formatted.push('\n');
                    newline_count += 1;
                }
            } else {
                newline_count = 0;

                if line.contains(':') {
                    seen_label = true;

                    // append a newline before if there isn't one already.
                    if !formatted.ends_with('\n') {
                        formatted.push('\n');
                    }

                    // No need to bump newline count b/c we add another line of non-newline text, so it'll be reset.
                }
                // Don't indent labels.
                else if seen_label {
                    formatted.push('\t');
                }

                formatted.push_str(&line);
                formatted.push('\n');
            }
        }

        formatted
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}
