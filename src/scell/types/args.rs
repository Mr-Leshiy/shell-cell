use std::collections::BTreeMap;

/// A map of additional key-value arguments supplied by the user during the startup
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct StartupArguments(BTreeMap<String, String>);

impl From<BTreeMap<String, String>> for StartupArguments {
    fn from(value: BTreeMap<String, String>) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for StartupArguments {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
