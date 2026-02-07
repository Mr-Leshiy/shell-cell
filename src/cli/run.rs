use crate::{
    buildkit::BuildKitD,
    cli::{Cli, progress::Progress},
    pty,
    scell::SCell,
};

impl Cli {
    pub async fn run(self) -> color_eyre::Result<()> {
        let mut pb = Progress::new(5)?;

        // STEP 1
        let scell = pb
            .run_step(
                format!(
                    "📝    Processing Shell-Cell source file '{}'...",
                    self.scell_path.display()
                ),
                async || SCell::compile(&self.scell_path, None),
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

        // STEP 5
        pb.run_step(
            "🚀    Starting 'Shell-Cell' session...".to_string(),
            async || {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                color_eyre::eyre::Ok(())
            },
        )
        .await?;

        pty::run(&pty).await?;

        println!("Finished 'Shell-Cell' session\n<Press any key to exit>");
        Ok(())
    }
}
