use std::path::{Path, PathBuf};

use test_case::test_case;

use crate::scell::{
    parser::{name::TargetName, shell::ShellStmt}, Link, SCell
};

#[test_case(
    "simple.yml", None 
    => SCell {
        links: vec![
            Link::Node {
                name: "main".parse().unwrap(),
                location: PathBuf::from("src/scell/compile/tests/simple.yml"),
                workspace: Default::default(),
                copy: Default::default(),
                build: Default::default(),
            },
            Link::Root("from".parse().unwrap())
            ],
        shell: ShellStmt {
            bin_path: "shell".to_string(),
            commands: vec![]
        },
        hang: "hang".to_string(),
    } 
    ; "simple"
)]
fn compile_ok_test(
    path: &str,
    target: Option<TargetName>,
) -> SCell {
    SCell::compile(Path::new("src/scell/compile/tests").join(path), target).unwrap()
}
