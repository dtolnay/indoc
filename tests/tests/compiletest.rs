// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "compiletest")]

extern crate compiletest_rs as compiletest;

use std::env;

fn run_dir(dir: &'static str) {
    let mut config = compiletest::Config::default();

    config.mode = compiletest::common::Mode::Ui;
    config.target_rustcflags = Some("-L ../target/debug/deps".to_owned());
    if let Ok(name) = env::var("TESTNAME") {
        config.filter = Some(name);
    }
    config.src_base = format!("tests/{}", dir).into();
    config.build_base = std::path::PathBuf::from("../target/ui");

    compiletest::run_tests(&config);
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
