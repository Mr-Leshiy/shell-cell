use comfy_table::Table;

use crate::{
    buildkit::{BuildKitD, container_info::ContainerInfo},
    cli::Cli,
};

impl Cli {
    pub async fn ls(self) -> anyhow::Result<()> {
        let buildkit = BuildKitD::start().await?;

        let containers = buildkit.list_containers().await?;
        let cotainer_info_to_row = |c: ContainerInfo| {
            [
                c.name,
                c.created_at
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
                c.status,
            ]
        };

        let mut table = Table::new();
        table
            .set_header(vec!["name", "created at", "status"])
            .add_rows(containers.into_iter().map(cotainer_info_to_row));

        println!("{table}");
        Ok(())
    }
}
