#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    if cfg!(feature = "unstable") {
        t.compile_fail("tests/ui-unstable/*.rs");
    } else {
        t.compile_fail("tests/ui-stable/*.rs");
    }
}
