use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
//use assert_fs::{TempDir, fixture::ChildPath};
use assert_fs::{TempDir, fixture::ChildPath};
use std::process::Command;
use std::path::PathBuf;

pub struct TestEnv {
    pub dir: TempDir,
    pub cmd: Command
}


impl TestEnv {
    pub fn new() -> TestEnv {
        let dir = TempDir::new().expect("Failed to create temp dir.");
        let mut cmd = Command::cargo_bin("pier").expect("Failed to set cargo binary pier");
        TestEnv {
            cmd, dir,
        }
    }
    pub fn create_config(&mut self, path: &str, content: &'static str) -> ChildPath {
        let config_file = self.dir.child(path);
        config_file.write_str(content).expect("Failed to content to file.");
        config_file
    }

    pub fn with_config(&mut self, content: &'static str) -> ChildPath {
        let path = self.create_config("pier_config", content);
        self.cmd.args(&["-c", path.path().to_str().unwrap()]);
        path
    }
    
    pub fn cmd(&mut self) -> &mut Command {
        &mut self.cmd
    }
    pub fn from_config(content: &'static str) -> (ChildPath, TestEnv) {
        let mut te = TestEnv::new();
        let cfg = te.with_config(content);
        (cfg, te)
    }

}

#[macro_export]
macro_rules! pier_test {
    (cfg => $content: expr, fn => $name:ident, $func:expr) => {
        #[test]
        fn $name() {
            let (mut cfg, mut te) = TestEnv::from_config($content);
            $func(cfg, te)
        }
    };
    ($content: expr, fn => $name:ident, cli => $cli:expr, $func:expr) => {
        #[test]
        fn $name() {
            let (mut cfg, mut te) = TestEnv::from_config($content);

            let args: Vec<&str> = $cli.split(" ").collect();
            for arg in args {
                te.cmd.arg(arg);
            };
            $func(cfg, te)
        }
    };
}
