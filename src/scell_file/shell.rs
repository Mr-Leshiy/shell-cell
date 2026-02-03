use std::fmt::Write;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShellStmt {
    pub bin_path: String,
    pub commands: Vec<String>,
}

impl ShellStmt {
    pub fn to_dockerfile(
        &self,
        dockerfile: &mut String,
    ) {
        let _ = writeln!(
            dockerfile,
            "SHELL [\"{}\", {}]",
            self.bin_path,
            self.commands.iter().map(|v| format!("\"{v}\"")).join(",")
        );
    }
}

impl<'de> serde::Deserialize<'de> for ShellStmt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let vec = Vec::<String>::deserialize(deserializer)?;
        let [shell, commands @ ..] = vec.as_slice() else {
            return Err(serde::de::Error::custom(
                "'shell' must have at least one etry, if its present",
            ));
        };

        Ok(Self {
            bin_path: shell.clone(),
            commands: commands.to_vec(),
        })
    }
}
