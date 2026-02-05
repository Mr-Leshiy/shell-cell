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
    "simple_default_target.yml", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple_default_target.yml"),
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
    "simple_other_target.yml", Some("other".parse().unwrap()) 
    => SCell {
        links: vec![
            Link::Node {
                name: "other".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple_other_target.yml"),
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

fn compile_ok_test(
    path: &str,
    target: Option<TargetName>,
) -> SCell {
    SCell::compile(Path::new("src/scell/compile/tests").join(path), target).unwrap()
}
