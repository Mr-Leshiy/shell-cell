use std::path::{Path, PathBuf};

use test_case::test_case;

use crate::scell::{
    Link, SCell,
    parser::{
        name::TargetName,
        target::{build::BuildStmt, copy::CopyStmt, shell::ShellStmt, workspace::WorkspaceStmt},
    },
};

#[test_case(
    "default_target", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/ok/default_target"),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
    }
    ; "default target"
)]
#[test_case(
    "other_target", Some("other".parse().unwrap()) 
    => SCell {
        links: vec![
            Link::Node {
                name: "other".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/ok/other_target"),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
    }
    ; "other target"
)]
#[test_case(
    "few_targets", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/ok/few_targets"),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/ok/few_targets"),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
    }
    ; "few targets"
)]
#[test_case(
    "ref_other_files", None 
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
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
    }
    ; "ref other files"
)]
fn compile_ok_test(
    dir_path: &str,
    target: Option<TargetName>,
) -> SCell {
    SCell::compile(
        Path::new("src/scell/compile/tests/ok").join(dir_path),
        target,
    )
    .unwrap()
}
