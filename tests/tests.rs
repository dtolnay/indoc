// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::default_config();
    config.mode = mode.parse().unwrap();
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.target_rustcflags = Some("--extern indoc=target/debug/libindoc.so"
        .to_owned());
    compiletest::run_tests(&config);
}

#[test]
fn run_pass() {
    run_mode("run-pass");
}

#[test]
fn compile_fail() {
    run_mode("compile-fail");
}
