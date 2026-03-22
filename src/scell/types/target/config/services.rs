use std::hash::Hash;

use crate::scell::types::target::TargetStmt;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ServicesStmt(pub Vec<TargetStmt>);

impl Hash for ServicesStmt {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        if !self.0.is_empty() {
            self.0.hash(state);
        }
    }
}
