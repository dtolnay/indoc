fn main() {
    // Warning: build.rs is not published to crates.io.

    println!("cargo:rustc-check-cfg=cfg(indoc_test_cstr)");

    if rustversion::cfg!(since(1.77)) {
        println!("cargo:rustc-cfg=indoc_test_cstr");
    }
}
