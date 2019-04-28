// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "compiletest")]

use compiletest_rs as compiletest;

fn run_dir(dir: &'static str) {
    compiletest::run_tests(&compiletest::Config {
        mode: compiletest::common::Mode::Ui,
        src_base: format!("tests/{}", dir).into(),
        target_rustcflags: Some(String::from(
            "\
             --edition=2018 \
             -L ../target/debug/deps \
             -Z unstable-options \
             --extern indoc \
             ",
        )),
        build_base: std::path::PathBuf::from("../target/ui"),
        ..Default::default()
    });
}

#[cfg(not(feature = "unstable"))]
#[test]
fn ui() {
    run_dir("ui-stable");
}

#[cfg(feature = "unstable")]
#[test]
fn ui() {
    run_dir("ui-unstable");
}
