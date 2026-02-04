use crate::{
    buildkit::BuildKitD,
    cli::{Cli, progress::Progress},
    pty,
    scell::SCell,
    scell_file::SCellFile,
};

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        let mut pb = Progress::new(5)?;

        // STEP 1
        let scell = pb
            .run_step(
                format!(
                    "📝    Processing Shell-Cell source file '{}'...",
                    self.scell_path.display()
                ),
                async || {
                    let scell_f = SCellFile::from_path(&self.scell_path)?;
                    SCell::compile(scell_f, None)
                },
            )
            .await?;

        // STEP 2
        let buildkit = pb
            .run_step(
                "📡    Connecting to the 'BuildKit'...".to_string(),
                async || BuildKitD::start().await,
            )
            .await?;

        // STEP 3
        pb.run_build_step(
            "📝    Building 'Shell-Cell' image...".to_string(),
            async |sp| {
                buildkit
                    .build_image(&scell, {
                        |msg| {
                            if self.verbose {
                                sp.println(format!("    {msg}"));
                            }
                        }
                    })
                    .await?;
                Ok(())
            },
        )
        .await?;

        // STEP 4
        let pty = pb
            .run_step(
                "🚀    Starting 'Shell-Cell' container...".to_string(),
                async || {
                    buildkit.start_container(&scell).await?;
                    buildkit.attach_to_shell(&scell).await
                },
            )
            .await?;

        pty::run(&pty).await?;

        // FINAL STEP
        pb.run_spinner(
            "🏁    Stopping 'Shell-Cell' container...".to_string(),
            async || buildkit.stop_container(&scell).await,
        )
        .await?;

        println!("Finished 'Shell-Cell' session\n<Press any key to exit>");
        Ok(())
    }
}
