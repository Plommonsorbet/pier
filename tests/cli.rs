#![feature(proc_macro_hygiene)]
use assert_cmd::prelude::*;
use std::process::Command; //use shell::cmd;
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
    cmd: Command
}

struct MyStruct {
    my_value: Option<String>
}

fn main() {
    let a = 1;
    unsafe {
    }
}



fn test1(var: String) {
}


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

fn setup_simple(from_config: &str) -> TestEnv {
    let mut te = TestEnv::new();
    te.add_config(from_config);
    te
}


macro_rules! pier_test {
    (config => $config:expr, $name:ident, $func:expr) => {
        #[test]
        fn $name() {
            $func(setup_simple($config));
        }
    };

    (config => $config:expr, $name:ident, $( $key:ident => $value:expr ),* ; $func:expr)  => { 
        #[test]
        fn $name() {
            let mut te = setup_simple($config);
            $( te.cmd.$key($value);)*;
            $func(te);
        }
    };
}

pier_test!(config => SUCCESS_CONFIG, test_case_1, arg => "list"; | mut te: TestEnv | {
    te.cmd.arg("list");
    te.cmd.assert().success();
    
});


pier_test!(config => SUCCESS_CONFIG, foo, args => &["test", "foo", "bar", "lol"]; |mut te: TestEnv| {
    println!("dir_path: {:?}", te.test_root());
});

