use indoc::panicdoc;
use std::panic::{self, UnwindSafe};

fn panic_message(f: impl FnOnce() + UnwindSafe) -> String {
    let payload = panic::catch_unwind(f).unwrap_err();
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_owned()
    } else {
        panic::resume_unwind(payload)
    }
}

#[test]
fn string() {
    let message = panic_message(|| {
        panicdoc! {"
            Unexpected output:

                formatting failed
            Try again"}
    });
    assert_eq!(
        message,
        "Unexpected output:\n\n    formatting failed\nTry again"
    );
}

#[test]
fn formatting_arguments() {
    let command = "rustfmt";
    let message = panic_message(|| {
        panicdoc! {"
            {command} exited with {}
                {output:?}",
            1,
            output = "formatting failed",
        }
    });
    assert_eq!(message, "rustfmt exited with 1\n    \"formatting failed\"");
}

#[test]
fn raw_string() {
    let message = panic_message(|| {
        panicdoc! {r#"
            Unexpected "output":
                C:\tools\{}"#,
            "rustfmt",
        }
    });
    assert_eq!(message, "Unexpected \"output\":\n    C:\\tools\\rustfmt");
}

#[test]
fn string_trailing_newline() {
    let message = panic_message(|| {
        panicdoc! {"
            Unexpected output:
                {}
            ",
            "formatting failed",
        }
    });
    assert_eq!(message, "Unexpected output:\n    formatting failed\n");
}
