use config::Config;
use serde::Deserialize;

const ENV_VAR_PREFIX: &str = "SHELL_CELL_DEBUGGER";

#[derive(Debug, Deserialize)]
pub struct DebuggerConfig {
    #[serde(default)]
    pub enabled: bool,
}

impl DebuggerConfig {
    pub fn init() -> color_eyre::Result<Self> {
        let res: DebuggerConfig = Config::builder()
            .add_source(config::Environment::with_prefix(ENV_VAR_PREFIX).try_parsing(true))
            .build()?
            .try_deserialize()?;
        Ok(res)
    }
}
