use std::path::Path;

use test_case::test_case;

use crate::scell::{SCell, types::name::TargetName};

#[test_case(
    "default_target", None
    ; "default target"
)]
#[test_case(
    "other_target", Some("other".parse().unwrap())
    ; "other target"
)]
#[test_case(
    "few_targets", None
    ; "few targets"
)]
#[test_case(
    "ref_other_files", None
    ; "ref other files"
)]
#[test_case(
    "workspace_stmt", None
    ; "workspace statement"
)]
#[test_case(
    "copy_stmt", None
    ; "copy statement"
)]
#[test_case(
    "build_stmt", None
    ; "build statement"
)]
#[test_case(
    "env_stmt", None
    ; "env statement"
)]
#[test_case(
    "all_stmts", None
    ; "all statements"
)]
#[test_case(
    "ports_config", None
    ; "ports config"
)]
#[test_case(
    "mounts_config", None
    ; "mounts config"
)]
#[test_case(
    "from_docker", None
    ; "from docker"
)]
fn compile_ok_test(
    dir_path: &str,
    target: Option<TargetName>,
) {
    SCell::compile(
        Path::new("src/scell/compile/tests/ok").join(dir_path),
        target,
    )
    .unwrap();
}
