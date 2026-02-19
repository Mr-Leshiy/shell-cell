use std::path::Path;

use test_case::test_case;

use crate::{
    error::UserError,
    scell::{
        SCell,
        compile::errors::{
            CircularTargets, CopySrcNotFound, DirNotFoundFromStmt, DockerfileNotFound,
            FileLoadFromStmt, MissingEntrypoint, MissingHangStmt, MissingShellStmt, MissingTarget,
        },
        types::name::TargetName,
    },
};

const ERR_FIXTURES: &str = "src/scell/compile/tests/err";

#[test_case(
    "missing_target", None
    => MissingTarget("missing_target".parse().unwrap(), std::fs::canonicalize(Path::new(ERR_FIXTURES).join("missing_target")).unwrap())
    ; "missing target"
)]
#[test_case(
    "circular_targets", None
    => CircularTargets("other".parse().unwrap(), std::fs::canonicalize(Path::new(ERR_FIXTURES).join("circular_targets/other")).unwrap())
    ; "circular targets"
)]
#[test_case(
    "missing_entrypoint", None
    => MissingEntrypoint(std::fs::canonicalize(Path::new(ERR_FIXTURES).join("missing_entrypoint")).unwrap(), "main".parse().unwrap())
    ; "missing entrypoint"
)]
#[test_case(
    "missing_shell_stmt", None
    => MissingShellStmt
    ; "missing shell stmt"
)]
#[test_case(
    "missing_hang_stmt", None
    => MissingHangStmt
    ; "missing hang stmt"
)]
#[test_case(
    "dockerfile_not_found", None
    => DockerfileNotFound(
        std::path::PathBuf::from("Dockerfile"),
        "main".parse().unwrap(),
        std::fs::canonicalize(Path::new(ERR_FIXTURES).join("dockerfile_not_found")).unwrap()
    )
    ; "dockerfile not found"
)]
#[test_case(
    "copy_src_not_found", None
    => CopySrcNotFound(
        std::path::PathBuf::from("nonexistent.txt"),
        "main".parse().unwrap(),
        std::fs::canonicalize(Path::new(ERR_FIXTURES).join("copy_src_not_found")).unwrap()
    )
    ; "copy src not found"
)]
#[test_case(
    "dir_not_found_from_stmt", None
    => DirNotFoundFromStmt(
        std::path::PathBuf::from("nonexistent/path"),
        "other".parse().unwrap(),
        std::fs::canonicalize(Path::new(ERR_FIXTURES).join("dir_not_found_from_stmt")).unwrap()
    )
    ; "dir not found from stmt"
)]
#[test_case(
    "file_load_from_stmt", None
    => FileLoadFromStmt(
        std::fs::canonicalize(Path::new(ERR_FIXTURES).join("file_load_from_stmt/empty_dir")).unwrap(),
        "other".parse().unwrap(),
        std::fs::canonicalize(Path::new(ERR_FIXTURES).join("file_load_from_stmt")).unwrap()
    )
    ; "file load from stmt"
)]
fn compile_err_test<E: std::error::Error + PartialEq + Sync + Send + 'static>(
    dir_path: &str,
    target: Option<TargetName>,
) -> E {
    let err =
        SCell::compile(Path::new(ERR_FIXTURES).join(dir_path), target).expect_err("Must fail");
    let err = err.downcast::<UserError>().expect("Must be a UserError");
    err.inner()
        .downcast::<E>()
        .expect("Must be correct error type")
}
