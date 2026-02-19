use std::hash::Hash;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize)]
pub struct BuildStmt(pub Vec<String>);

impl Hash for BuildStmt {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        if !self.0.is_empty() {
            self.0.hash(state);
        }
    }
}
