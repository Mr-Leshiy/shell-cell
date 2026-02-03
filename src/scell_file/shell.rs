#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShellDef {
    pub bin_path: String,
    pub commands: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for ShellDef {
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
