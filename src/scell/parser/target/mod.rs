pub mod build;
pub mod config;
pub mod copy;
pub mod env;
pub mod from;
pub mod image;
pub mod shell;
pub mod workspace;

use self::{
    build::BuildStmt, config::ConfigStmt, copy::CopyStmt, from::FromStmt, shell::ShellStmt,
    workspace::WorkspaceStmt,
};
use crate::scell::parser::target::env::EnvStmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct TargetStmt {
    pub from: FromStmt,
    #[serde(default)]
    pub workspace: WorkspaceStmt,
    #[serde(default)]
    pub build: BuildStmt,
    #[serde(default)]
    pub copy: CopyStmt,
    #[serde(default)]
    pub env: EnvStmt,
    pub shell: Option<ShellStmt>,
    pub hang: Option<String>,
    pub config: Option<ConfigStmt>,
}
