//#![feature(proc_macro_hygiene)]
use assert_cmd::prelude::*;
use std::process::Command; //use shell::cmd;
//use utils::{config};
//use command_macros::command;
//use std::fs;
//use std::io;
//use std::path::PathBuf;
//use tempfile::{tempdir, TempDir, tempfile, NamedTempFile};
//use maplit::hashmap;

const SUCCESS_CONFIG: &str = "tests/assets/success.config.toml";
const FAIL_CONFIG: &str = "tests/assets/success.config.toml";
const PIER_BIN: &str = "target/debug/pier";


//pier_test!(test_case_1, cfg => SUCCESS_CONFIG, | mut te: TestEnv | {
//    te.cmd.arg("list");
//    te.cmd.assert().success();
//    
//});



//#[test]
//fn test_1() {
//    config!("test_path", {"test" => "fest", "foo"=> "bar"})
//}
