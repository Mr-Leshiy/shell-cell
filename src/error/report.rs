#[derive(Debug, Default)]
pub struct Report(Option<color_eyre::eyre::Report>);

impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_error<E: Into<color_eyre::eyre::Error>>(
        &mut self,
        err: E,
    ) {
        self.0 = match self.0.take() {
            Some(e) => Some(e.wrap_err(err.into())),
            None => Some(err.into()),
        };
    }

    pub fn check(self) -> color_eyre::Result<()> {
        match self.0 {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}
