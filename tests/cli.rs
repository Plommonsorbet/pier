#![feature(proc_macro_hygiene)]
use assert_cmd::prelude::*;
use std::process::Command;
//use shell::cmd;
use command_macros::command;

const SUCCESS_CONFIG: &str = "tests/assets/success.config.toml";
const FAIL_CONFIG: &str = "tests/assets/success.config.toml";
const PIER_BIN: &str = "target/debug/pier";

#[test]
fn test_cli() {
    let mut cmd = command!(
        (PIER_BIN)
        -c (SUCCESS_CONFIG)
        list
    );
    let assert = cmd.assert();

    assert
        .success();
    
}
