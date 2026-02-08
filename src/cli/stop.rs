use crate::{buildkit::BuildKitD, cli::Cli, scell::container_info::Status};

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

        let mut stopped = 0;
        for container in &running {
            buildkit
                .stop_container_by_name(&container.container_name)
                .await?;
            println!("Stopped container: {}", container.container_name);
            stopped += 1;
        }

        println!("Stopped {stopped}/{} running container(s).", running.len());
        Ok(())
    }
}
