use std::path::{Path, PathBuf};

use test_case::test_case;

use crate::scell::{
    Link, SCell, SCellInner,
    types::{
        name::TargetName,
        target::{
            build::BuildStmt,
            config::{
                ConfigStmt,
                mounts::MountsStmt,
                ports::{PortItem, PortProtocol, PortsStmt},
            },
            copy::{CopyStmt, CopyStmtEntry},
            env::{EnvStmt, EnvStmtItem},
            shell::ShellStmt,
            workspace::WorkspaceStmt,
        },
    },
};

#[test_case(
    "default_target", None
    => (SCell(SCellInner {
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
    }),
    "02679853b28a6ec1".to_string())
    ; "default target"
)]
#[test_case(
    "other_target", Some("other".parse().unwrap())
    => (SCell(SCellInner {
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
    }),
    "6cc8af376a928d25".to_string())
    ; "other target"
)]
#[test_case(
    "few_targets", None
    => (SCell(SCellInner {
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
    }),
    "540bfac5faeed0ad".to_string())
    ; "few targets"
)]
#[test_case(
    "ref_other_files", None
    => (SCell(SCellInner {
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
    }),
    "c36031f85cdf98f3".to_string())
    ; "ref other files"
)]
#[test_case(
    "workspace_stmt", None
    => (SCell(SCellInner {
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
    }),
    "97c3c56438147a50".to_string())
    ; "workspace statement"
)]
#[test_case(
    "copy_stmt", None
    => (SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/copy_stmt").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt(vec![
                    CopyStmtEntry {
                        src: vec![std::fs::canonicalize("src/scell/compile/tests/ok/copy_stmt/copy_file.txt").unwrap()],
                        dest: PathBuf::from("."),
                    },
                    CopyStmtEntry {
                        src: vec![std::fs::canonicalize("src/scell/compile/tests/ok/copy_stmt/copy-source").unwrap()],
                        dest: PathBuf::from("."),
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
    }),
    "a1aa6791e506577c".to_string())
    ; "copy statement"
)]
#[test_case(
    "build_stmt", None
    => (SCell(SCellInner {
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
    }),
    "3ae4db9b81d86613".to_string())
    ; "build statement"
)]
#[test_case(
    "env_stmt", None
    => (SCell(SCellInner {
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
    }),
    "a86022c340a12704".to_string())
    ; "env statement"
)]
#[test_case(
    "all_stmts", None
    => (SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/all_stmts").unwrap(),
                workspace: WorkspaceStmt(Some("/app".to_string())),
                copy: CopyStmt(vec![
                    CopyStmtEntry {
                        src: vec![std::fs::canonicalize("src/scell/compile/tests/ok/all_stmts/copy_file.txt").unwrap()],
                        dest: PathBuf::from("."),
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
    }),
    "b24d0a0a60fa4529".to_string())
    ; "all statements"
)]
#[test_case(
    "ports_config", None
    => (SCell(SCellInner {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: std::fs::canonicalize("src/scell/compile/tests/ok/ports_config").unwrap(),
                workspace: WorkspaceStmt::default(),
                copy: CopyStmt::default(),
                build: BuildStmt::default(),
                env: EnvStmt::default(),
            },
            Link::Root("from".parse().unwrap())
        ],
        shell: ShellStmt("shell".to_string()),
        hang: "hang".to_string(),
        config: Some(ConfigStmt {
            mounts: MountsStmt::default(),
            ports: PortsStmt(vec![
                PortItem { host_ip: None, host_port: "8080".to_string(), container_port: "80".to_string(), protocol: PortProtocol::Tcp },
                PortItem { host_ip: Some("127.0.0.1".to_string()), host_port: "9000".to_string(), container_port: "9000".to_string(), protocol: PortProtocol::Tcp },
                PortItem { host_ip: None, host_port: "6060".to_string(), container_port: "6060".to_string(), protocol: PortProtocol::Udp },
            ]),
        }),
    }),
    "e85d0536da0bd388".to_string())
    ; "ports config"
)]
fn compile_ok_test(
    dir_path: &str,
    target: Option<TargetName>,
) -> (SCell, String) {
    let scell = SCell::compile(
        Path::new("src/scell/compile/tests/ok").join(dir_path),
        target,
    )
    .unwrap();
    let scell_hash = scell.hex_hash().unwrap();
    (scell, scell_hash)
}
