// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Compute the maximal number of spaces that can be removed from every line, and
// remove them.
pub fn unindent(s: &str) -> String {
    // Document may start either on the same line as opening quote or
    // on the next line
    let ignore_first_line = s.starts_with('\n') ||
                            s.starts_with("\r\n");

    // Largest number of spaces that can be removed from every
    // non-whitespace-only line after the first
    let spaces = s.lines()
        .skip(1)
        .filter_map(count_spaces)
        .min()
        .unwrap_or(0);

    let mut result = String::with_capacity(s.len());
    for (i, line) in s.lines().enumerate() {
        if i > 1 || (i == 1 && !ignore_first_line) {
            result.push_str("\n");
        }
        if i == 0 {
            // Do not un-indent anything on same line as opening quote
            result.push_str(line);
        } else if line.len() > spaces {
            // Whitespace-only lines may have fewer than the number of spaces
            // being removed
            result.push_str(&line[spaces..]);
        }
    }
    result
}

// Number of leading spaces in the line, or None if the line is entirely spaces.
fn count_spaces(line: &str) -> Option<usize> {
    for (i, ch) in line.chars().enumerate() {
        if ch != ' ' {
            return Some(i);
        }
    }
    None
}
