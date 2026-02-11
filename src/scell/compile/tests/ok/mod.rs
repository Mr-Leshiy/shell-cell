use std::path::Path;

use test_case::test_case;

use crate::scell::{
    Link, SCell, SCellInner,
    parser::{
        name::TargetName,
        target::{
            build::BuildStmt, copy::CopyStmt, env::EnvStmt, shell::ShellStmt,
            workspace::WorkspaceStmt,
        },
    },
};

#[test_case(
    "default_target", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/default_target").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
    ; "default target"
)]
#[test_case(
    "other_target", Some("other".parse().unwrap())
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/other_target").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
    ; "other target"
)]
#[test_case(
    "few_targets", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/few_targets").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/few_targets").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
    ; "few targets"
)]
#[test_case(
    "ref_other_files", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/ref_other_files").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/ref_other_files/other").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Node {
                name: "other".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/few_targets").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
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
