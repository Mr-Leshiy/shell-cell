pub mod build;
pub mod config;
pub mod copy;
pub mod env;
pub mod from;
pub mod shell;
pub mod workspace;
pub mod hang;

use self::{
    build::BuildStmt, config::ConfigStmt, copy::CopyStmt, from::FromStmt, shell::ShellStmt,
    workspace::WorkspaceStmt,
};
use crate::scell::types::target::{env::EnvStmt, hang::HangStmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct TargetStmt {
    #[serde(flatten)]
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
    pub hang: Option<HangStmt>,
    pub config: Option<ConfigStmt>,
}
