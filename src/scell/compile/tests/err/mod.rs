use std::{marker::PhantomData, path::Path};

use test_case::test_case;

use crate::{
    error::UserError,
    scell::{compile::errors::{CircularTargets, MissingTarget}, parser::name::TargetName, SCell},
};

// TODO add test cases for `DirNotFoundFromStmt` and `FileLoadFromStmt`
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
