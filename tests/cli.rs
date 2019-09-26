#![feature(proc_macro_hygiene)]
use assert_cmd::prelude::*;
use std::process::Command;
//use shell::cmd;
use command_macros::command;
use std::fs;
use std::io;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir, tempfile, NamedTempFile};

const SUCCESS_CONFIG: &str = "tests/assets/success.config.toml";
const FAIL_CONFIG: &str = "tests/assets/success.config.toml";
const PIER_BIN: &str = "target/debug/pier";

struct TestEnv {
    dir: TempDir,
}

impl TestEnv {
    fn new() -> TestEnv {
        TestEnv {
            dir: tempdir().expect("Failed to create temp dir."),
        }
    }
    fn tempfile_from(&self, from_file: &str, to_filename: &str) -> PathBuf {
        let temp_file = self.dir.path().join(to_filename);
        fs::copy(from_file, &temp_file).expect("Copy file to tempfile");
        temp_file
    }
}

fn rfile(path: &NamedTempFile) -> String {
    fs::read_to_string(path).expect("failed to read file.")
}

fn pfile(path: &NamedTempFile) {
    println!("{}", rfile(path))
}
fn tmpf(from_file: &str) -> NamedTempFile {
    let tmp = NamedTempFile::new().expect("failed to make tempfile.");
    fs::copy(from_file, &tmp.path()).expect("Copy file to tempfile");
    tmp
}


//#[test]
//fn test_cli() {
//    let tmp = tmpf(SUCCESS_CONFIG);
//    println!("temp file: {:?}", tmp);
//    pfile(tmp.path().to_path_buf());
//    let test_env = TestEnv::new();
//    let config = test_env.tempfile_from(SUCCESS_CONFIG, "pier_config");
//    let mut cmd = command!(
//        (PIER_BIN)
//        -c (config)
//        list
//    );
//    let assert = cmd.assert();
//
//    assert.failure();
//}

#[test]
fn test_add() {
    let config = tmpf(SUCCESS_CONFIG);
    println!("Before:");
    pfile(&config);
    let mut cmd = command!(
        (PIER_BIN)
        -c (config.path().to_path_buf())
        add "echo yes" -a yes_man
    );

    let assert = cmd.assert();

    assert.failure();
    println!("After:");
    pfile(&config);

}

#[test]
fn test_() {

}
