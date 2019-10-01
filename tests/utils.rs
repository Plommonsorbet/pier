use assert_cmd::prelude::*;
use std::process::Command;
use std::fs;
use std::io;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir, tempfile, NamedTempFile};
use maplit::hashmap;

struct TestEnv {
    dir: TempDir,
    cmd: Command
}

const SUCCESS_CONFIG: &str = "tests/assets/success.config.toml";
const FAIL_CONFIG: &str = "tests/assets/success.config.toml";
const PIER_BIN: &str = "target/debug/pier";

impl TestEnv {
    fn new() -> TestEnv {
        let dir = tempdir().expect("Failed to create temp dir.");
        let mut cmd = Command::cargo_bin("pier").expect("Failed to set cargo binary pier");
        cmd.current_dir(&dir);
        TestEnv {
            cmd, dir 
        }
    }

    fn cmd(&mut self) -> &mut Command {
        &mut self.cmd
    }

    fn test_root(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }
    fn join_path(&self, filename: &str) -> PathBuf {
        self.dir.path().join(filename).to_path_buf()
    }
    fn copy_from(&self, from_file: &str, to_filename: &str) {
        fs::copy(from_file, self.join_path(to_filename)).expect("Copy file to tempfile");
    }

    fn add_config(&mut self, config_from: &str) {
        self.copy_from(config_from, "config");
        self.cmd.args(&["-c", "config"]);
        
    }
}

//macro_rules! pier_test {
//    ($name:ident, cfg => $config:expr, $func:expr) => {
//        #[test]
//        fn $name() {
//            let te = setup_simple($config);
//            $func(te);
//        }
//    };
//
//}
macro_rules! config {
    ($path:expr; $($key:expr => $value:expr,)+) => { 
        println!("path: {}, {:?}", $path,hashmap!($($key => $value),+));
    };
}

#[test]
fn test_1() {
    config!("test_path"; "test" => "fest", "foo" => "bar");
    assert!(false);
}

fn setup_simple(from_config: &str) -> TestEnv {
    let mut te = TestEnv::new();
    te.add_config(from_config);
    te
}

