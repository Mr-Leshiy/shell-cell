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
    "simple_default_target", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple_default_target"),
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
    ; "simple default target"
)]
#[test_case(
    "simple_other_target", Some("other".parse().unwrap()) 
    => SCell {
        links: vec![
            Link::Node {
                name: "other".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple_other_target"),
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
    ; "simple other target"
)]
#[test_case(
    "simple_few_targets", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple_few_targets"),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple_few_targets"),
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
    ; "simple few targets"
)]
#[test_case(
    "simple_ref_other_files", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple_ref_other_files"),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/simple_ref_other_files/other").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/simple_few_targets").unwrap(),
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
    ; "simple ref other files"
)]
fn compile_ok_test(
    dir_path: &str,
    target: Option<TargetName>,
) -> SCell {
    SCell::compile(Path::new("src/scell/compile/tests").join(dir_path), target).unwrap()
}
