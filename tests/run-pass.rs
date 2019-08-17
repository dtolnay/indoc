#![cfg_attr(feature = "unstable", feature(proc_macro_hygiene))]
#![cfg_attr(rustfmt, rustfmt_skip)]

use indoc::indoc;

#[test]
fn byte_string() {
    let indoc = indoc!(b"
        a

            \\b
        c");
    let expected = b"a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn carriage_return() {
    // Every line in the string ends with \r\n
    let indoc = indoc!("
        a

            \\b
        c");
    let expected = "a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn empty_string() {
    let indoc = indoc!("");
    let expected = "";
    assert_eq!(indoc, expected);
}

#[test]
fn joined_first_line() {
    let indoc = indoc!("\
        a");
    let expected = "a";
    assert_eq!(indoc, expected);
}

#[test]
fn joined_lines() {
    let indoc = indoc!("
        a\
        b
        c\
          d
        e");
    let expected = "ab\ncd\ne";
    assert_eq!(indoc, expected);
}

#[test]
fn no_leading_newline() {
    let indoc = indoc!("a
                        b
                        c");
    let expected = "a\nb\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn one_line() {
    let indoc = indoc!("a");
    let expected = "a";
    assert_eq!(indoc, expected);
}

#[test]
fn raw_byte_string() {
    let indoc = indoc!(br#"
        "a"

            \\b
        c"#);
    let expected = b"\"a\"\n\n    \\\\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn raw_string() {
    let indoc = indoc!(r#"
        "a"

            \\b
        c"#);
    let expected = "\"a\"\n\n    \\\\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn string() {
    let indoc = indoc!("
        a

            \\b
        c");
    let expected = "a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn string_trailing_newline() {
    let indoc = indoc!("
        a

            \\b
        c
    ");
    let expected = "a\n\n    \\b\nc\n";
    assert_eq!(indoc, expected);
}

#[test]
fn trailing_whitespace() {
    let indoc = indoc!("
        2 below
          
        0 below
        
        -2 below
      
        end");
    let expected = "2 below\n  \n0 below\n\n-2 below\n\nend";
    assert_eq!(indoc, expected);
}

#[cfg(feature = "unstable")]
#[test]
fn indoc_as_format_string() {
    let s = format!(indoc!("{}"), true);
    assert_eq!(s, "true");
}
