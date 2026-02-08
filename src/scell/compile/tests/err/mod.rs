use std::{marker::PhantomData, path::Path};

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

// TODO replace `PhantomData` with returning an error object, instead of just a type
#[test_case(
    "missing_target", None 
    => PhantomData::<MissingTarget>
    ; "missing target"
)]
#[test_case(
    "circular_targets", None
    => PhantomData::<CircularTargets>
    ; "circular targets"
)]
#[test_case(
    "missing_entrypoint", None
    => PhantomData::<MissingEntrypoint>
    ; "missing entrypoint"
)]
#[test_case(
    "missing_shell_stmt", None
    => PhantomData::<MissingShellStmt>
    ; "missing shell stmt"
)]
#[test_case(
    "missing_hang_stmt", None
    => PhantomData::<MissingHangStmt>
    ; "missing hang stmt"
)]
fn compile_err_test<E: std::error::Error + Sync + Send + 'static>(
    dir_path: &str,
    target: Option<TargetName>,
) -> PhantomData<E> {
    let err = SCell::compile(
        Path::new("src/scell/compile/tests/err").join(dir_path),
        target,
    )
    .expect_err("Must fail");
    let err = err.downcast::<UserError>().expect("Must be a UserError");
    assert!(err.inner().is::<E>());
    PhantomData::<E>
}
