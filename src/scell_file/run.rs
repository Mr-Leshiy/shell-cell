use std::fmt::Write;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct RunStmt(pub Vec<String>);

impl RunStmt {
    pub fn to_dockerfile(
        &self,
        dockerfile: &mut String,
    ) {
        for e in &self.0 {
            let _ = writeln!(dockerfile, "RUN {e}");
        }
    }
}
