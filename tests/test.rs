#![feature(plugin)]
#![plugin(indoc)]

#[test]
fn indoc_string_with_trailing_newline() {
    let indoc = indoc!("
        a

            \\b
        c
    ");
    let expected = "a\n\n    \\b\nc\n";
    assert_eq!(indoc, expected);
}

#[test]
fn indoc_string_without_trailing_newline() {
    let indoc = indoc!("
        a

            \\b
        c");
    let expected = "a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn indoc_raw_string() {
    let indoc = indoc!(r#"
        "a"

            \\b
        c"#);
    let expected = "\"a\"\n\n    \\\\b\nc";
    assert_eq!(indoc, expected);
}
