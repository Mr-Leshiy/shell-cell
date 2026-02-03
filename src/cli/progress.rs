use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};

pub struct Progress {
    multi: MultiProgress,
    common_style: ProgressStyle,
    spinner_style: ProgressStyle,
    total_steps: u64,
    current_step: u64,
}

impl Progress {
    pub fn new(total_steps: u64) -> anyhow::Result<Self> {
        Ok(Self {
            multi: MultiProgress::with_draw_target(ProgressDrawTarget::stdout()),
            common_style: ProgressStyle::with_template("[{pos}/{len}] {msg}")?,
            spinner_style: ProgressStyle::with_template("{spinner} {msg}")?,
            total_steps,
            current_step: 0,
        })
    }

    pub async fn run_spinner<F, T>(
        &mut self,
        msg: String,
        f: F,
    ) -> anyhow::Result<T>
    where
        F: AsyncFnOnce() -> anyhow::Result<T>,
    {

        let pb = self.multi.add(ProgressBar::new_spinner());
        pb.set_style(self.spinner_style.clone());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message(msg);

        let result = f().await;
        pb.finish_and_clear();
        result
    }

    pub async fn run_step<F, T>(
        &mut self,
        msg: String,
        f: F,
    ) -> anyhow::Result<T>
    where
        F: AsyncFnOnce() -> anyhow::Result<T>,
    {
        self.current_step = self.current_step.saturating_add(1);

        let pb = self.multi.add(ProgressBar::new(self.total_steps));
        pb.set_style(self.common_style.clone());
        pb.set_position(self.current_step);
        pb.set_message(msg);

        let result = f().await;
        pb.finish();
        pb.set_position(self.current_step);
        result
    }

    /// Special step that includes a sub-spinner for long-running build tasks
    pub async fn run_build_step<F, T>(
        &mut self,
        msg: String,
        f: F,
    ) -> anyhow::Result<T>
    where
        F: AsyncFnOnce(ProgressBar) -> anyhow::Result<T>,
    {
        self.current_step = self.current_step.saturating_add(1);

        let pb = self.multi.add(ProgressBar::new(self.total_steps));
        pb.set_style(self.common_style.clone());
        pb.set_position(self.current_step);
        pb.set_message(msg.clone());

        let spinner = self.multi.add(ProgressBar::new_spinner());
        spinner.set_style(self.spinner_style.clone());
        spinner.enable_steady_tick(Duration::from_millis(100));

        let result = f(spinner.clone()).await?;
        spinner.finish_and_clear();
        pb.finish();
        pb.set_position(self.current_step);
        Ok(result)
    }
}
