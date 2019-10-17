use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use assert_fs::fixture::ChildPath;

mod test_utils;
use test_utils::*;

const CONFIG_OK_1: &'static str = r#"
[scripts.read-poem-from-file]
alias = "read-poem"
command = "cat tests/assets/poem.txt"

[scripts.print-env-var]
alias = "print-var"
command = "echo $MYVAR"

[scripts.touch-tmp]
alias = "touch-tmp"
description = "This command creates a temporary file."
command = "touch test.tmp"
"#;


pier_test!(cfg => CONFIG_OK_1, fn => test_list_scripts, | cfg: ChildPath, mut te: TestEnv | {
    // TODO Add some way to order the list output so it matches?
    te.cmd.arg("list");
    te.cmd.assert().success();
});


pier_test!(cfg => CONFIG_OK_1, fn => test_add_script, | cfg: ChildPath, mut te: TestEnv | {
    te.cmd.args(&["add", r#"cat /etc/issue"#, "-a", "list-distro"]);
    te.cmd.assert().success();
    cfg.assert(predicate::str::contains(
r#"[scripts.list-distro]
alias = "list-distro"
command = "cat /etc/issue""#
    ));
});

pier_test!(cfg => 
r#"
[scripts.test_run]
alias = "test_run"
command = '''
echo 'test'
'''"#, fn => test_run_script_1, | cfg: ChildPath, mut te: TestEnv | {
    te.cmd.args(&["run", "test_run"]);
    te.cmd.assert().failure();
    //te.cmd.assert().success();
});


//pier_test!(cfg => CONFIG_OK_1, fn => test_remove_script, | cfg: ChildPath, mut te: TestEnv | {
//    te.cmd.args(&["remove", "touch-tmp"]);
//    te.cmd.assert().success();
//    cfg.assert(predicate::str::similar( r#"[scripts.read-poem-from-file]
//alias = "read-poem"
//command = "cat tests/assets/poem.txt"
//
//[scripts.print-env-var]
//alias = "print-var"
//command = "echo $MYVAR""#
//    ).trim());
//});

//const CONFIG_OK_RUN: &'static str = r#"
//[scripts.test_1]
//alias = "test_1"
//command = '''echo 'test''''
//"#;

