use std::path::{Path, PathBuf};

use test_case::test_case;

use crate::scell::{
    Link, SCell,
    parser::{
        build::BuildStmt, copy::CopyStmt, name::TargetName, shell::ShellStmt,
        workspace::WorkspaceStmt,
    },
};

#[test_case(
    "circular_targets", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/ok/ref_other_files"),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/ref_other_files/other").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/few_targets").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt {
            bin_path: "shell".to_string(),
            commands: vec![]
        },
        hang: "hang".to_string(),
    }
    ; "circular targets"
)]
fn compile_err_test(
    dir_path: &str,
    target: Option<TargetName>,
) -> SCell {
    SCell::compile(Path::new("src/scell/compile/tests/err").join(dir_path), target).unwrap()
}
