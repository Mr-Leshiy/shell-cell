//! Headless implementation of `scell stop`: stops all running Shell-Cell
//! containers and reports outcomes line by line on stdout/stderr.

use crate::buildkit::{
    BuildKitD,
    container_info::{SCellContainerInfo, Status},
};

pub async fn stop(buildkit: &BuildKitD) -> color_eyre::Result<()> {
    let containers = buildkit.list_containers().await?;
    let running: Vec<_> = containers
        .into_iter()
        .filter(|c| c.status == Status::Running)
        .collect();

    if running.is_empty() {
        println!("No running Shell-Cell containers");
        return Ok(());
    }

    let mut stopped: usize = 0;
    let mut failed: usize = 0;
    for c in &running {
        let name = SCellContainerInfo::container_name(&c.id, c.service_name.as_ref());
        match buildkit.stop_container(c).await {
            Ok(()) => {
                println!("stopped {name}");
                stopped = stopped.saturating_add(1);
            },
            Err(e) => {
                eprintln!("error: failed to stop {name}: {e}");
                failed = failed.saturating_add(1);
            },
        }
    }
    println!("stopped {stopped}, failed {failed}");
    if failed > 0 {
        color_eyre::eyre::bail!("{failed} container(s) failed to stop");
    }
    Ok(())
}
