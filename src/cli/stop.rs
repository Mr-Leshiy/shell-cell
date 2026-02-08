use crate::{buildkit::BuildKitD, cli::Cli, error::Report, scell::container_info::Status};

impl Cli {
    pub async fn stop(self) -> color_eyre::Result<()> {
        let buildkit = BuildKitD::start().await?;

        let containers = buildkit.list_containers().await?;
        let running: Vec<_> = containers
            .into_iter()
            .filter(|c| c.status == Status::Running)
            .collect();

        if running.is_empty() {
            println!("No running Shell-Cell containers found.");
            return Ok(());
        }

        let mut report = Report::new();
        for container in &running {
            if let Err(err) = buildkit
                .stop_container_by_name(&container.container_name)
                .await
            {
                report.add_error(err);
            } else {
                println!("Stopped container: {}", container.container_name);
            }
        }
        report.check()
    }
}
