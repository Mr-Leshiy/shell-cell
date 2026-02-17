use std::path::{Path, PathBuf};

use test_case::test_case;

use crate::scell::{
    Link, SCell, SCellInner,
    types::{
        name::TargetName,
        target::{
            build::BuildStmt,
            copy::{CopyStmt, CopyStmtEntry},
            env::{EnvStmt, EnvStmtItem},
            shell::ShellStmt,
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
#[test_case(
    "workspace_stmt", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/workspace_stmt").unwrap(),
                workspace: WorkspaceStmt(Some("/workspace".to_string())),
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
    ; "workspace statement"
)]
#[test_case(
    "copy_stmt", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/copy_stmt").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt(vec![
                    CopyStmtEntry {
                        src: vec![std::fs::canonicalize("src").unwrap()],
                        dest: PathBuf::from("dst"),
                    },
                    CopyStmtEntry {
                        src: vec![std::fs::canonicalize("Cargo.toml").unwrap()],
                        dest: PathBuf::from("dst2"),
                    },
                ]),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
    ; "copy statement"
)]
#[test_case(
    "build_stmt", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/build_stmt").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt(vec![
                    "apt-get update".to_string(),
                    "apt-get install -y curl".to_string(),
                ]),
                env: EnvStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
    ; "build statement"
)]
#[test_case(
    "env_stmt", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/env_stmt").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt(vec![
                    EnvStmtItem { key: "DB_HOST".to_string(), value: "localhost".to_string() },
                    EnvStmtItem { key: "PORT".to_string(), value: "8080".to_string() },
                ]),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
    ; "env statement"
)]
#[test_case(
    "all_stmts", None
    => SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/all_stmts").unwrap(),
                workspace: WorkspaceStmt(Some("/app".to_string())),
                copy: CopyStmt(vec![
                    CopyStmtEntry {
                        src: vec![std::fs::canonicalize("src").unwrap()],
                        dest: PathBuf::from("/app/src"),
                    },
                ]),
                build: BuildStmt(vec![
                    "apt-get update".to_string(),
                    "apt-get install -y git".to_string(),
                ]),
                env: EnvStmt(vec![
                    EnvStmtItem { key: "MY_VAR".to_string(), value: "hello".to_string() },
                    EnvStmtItem { key: "PATH".to_string(), value: "/usr/local/bin:/usr/bin".to_string() },
                ]),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Option::default(),
    })
    ; "all statements"
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
