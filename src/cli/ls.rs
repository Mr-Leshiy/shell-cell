use comfy_table::Table;

use crate::{buildkit::BuildKitD, cli::Cli, scell::SCellContainerInfo};

impl Cli {
    pub async fn ls(self) -> anyhow::Result<()> {
        let buildkit = BuildKitD::start().await?;

        let containers = buildkit.list_containers().await?;
        let cotainer_info_to_row = |c: SCellContainerInfo| {
            [
                c.name.to_string(),
                format!("{}", c.location.display()),
                c.created_at
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
                c.container_name,
                c.status,
            ]
        };

        let mut table = Table::new();
        table
            .set_header(vec![
                "name",
                "source location",
                "created at",
                "container name",
                "status",
            ])
            .add_rows(containers.into_iter().map(cotainer_info_to_row));

        println!("{table}");
        Ok(())
    }
}
