//! Command Line Interface implementation

mod progress;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{
    buildkit::BuildKitD, cli::progress::Progress, pty, scell::SCell, scell_file::SCellFile,
};

#[allow(clippy::doc_markdown)]
/// Binary build info
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser)]
#[clap(version = built_info::PKG_VERSION)]
#[clap(about = built_info::PKG_DESCRIPTION)]
pub struct Cli {
    /// Path to the 'scell.yml' file (Optional),
    #[clap(value_name = "FILE", default_value = "./scell.yml")]
    pub scell_path: PathBuf,

    /// Show detailed logs
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all existing Shell-Cell containers
    Ls,
}

impl Cli {
    pub async fn exec(self) -> anyhow::Result<()> {
        if self.command.is_none() {
            let verbose = self.verbose;
            self.run().await.map_err(|e| {
                if verbose {
                    e
                } else {
                    anyhow::anyhow!("{e}\n To enable verbose output use -v, --verbose flags")
                }
            })?;
        }

        Ok(())
    }

    async fn run(self) -> anyhow::Result<()> {
        let mut pb = Progress::new(4)?;

        // STEP 1
        let scell = pb
            .run_step(
                format!(
                    "📝    Processing Shell-Cell source file '{}'...",
                    self.scell_path.display()
                ),
                async || {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    let scell_f = SCellFile::from_path(&self.scell_path)?;
                    SCell::compile(scell_f, self.scell_path, None)
                },
            )
            .await?;

        // STEP 2
        let buildkit = pb
            .run_step(
                "📡    Connecting to the BuildKit...".to_string(),
                async || {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    BuildKitD::start().await
                },
            )
            .await?;

        // STEP 3
        pb.run_build_step(
            "📝    Building Shell-Cell image...".to_string(),
            async |sp| {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                buildkit
                    .build_image(&scell, {
                        |msg| {
                            if self.verbose {
                                sp.println(format!("    {msg}"));
                            }
                        }
                    })
                    .await?;
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                Ok(())
            },
        )
        .await?;

        // STEP 4
        let pty = pb
            .run_step(
                "🚀    Starting Shell-Cell container...".to_string(),
                async || {
                    buildkit.start_container(&scell).await?;
                    buildkit.attach_to_shell(&scell).await
                },
            )
            .await?;
        pty::run(&pty).await?;

        Ok(())
    }
}
