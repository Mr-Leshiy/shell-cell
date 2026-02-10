use std::path::Path;

use test_case::test_case;

use crate::{
    error::UserError,
    scell::{
        SCell,
        compile::errors::{
            CircularTargets, MissingEntrypoint, MissingHangStmt, MissingShellStmt, MissingTarget,
        },
        parser::name::TargetName,
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
