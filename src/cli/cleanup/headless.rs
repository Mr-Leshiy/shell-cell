//! Headless implementation of `scell cleanup`: removes orphan (or all when
//! `--all`) Shell-Cell containers and images, reporting outcomes on
//! stdout/stderr.

use crate::buildkit::{
    BuildKitD, container_info::SCellContainerInfo, image_info::SCellImageInfo,
};

pub async fn cleanup(
    buildkit: &BuildKitD,
    all: bool,
) -> color_eyre::Result<()> {
    let containers = buildkit.list_containers().await?;
    let images = buildkit.list_images().await?;

    let containers: Vec<_> = if all {
        containers
    } else {
        containers.into_iter().filter(|c| c.orphan).collect()
    };
    let images: Vec<_> = if all {
        images
    } else {
        images.into_iter().filter(|i| i.orphan).collect()
    };

    if containers.is_empty() && images.is_empty() {
        println!("Nothing to clean up");
        return Ok(());
    }

    let mut failed: usize = 0;
    for c in &containers {
        let name = SCellContainerInfo::container_name(&c.id, c.service_name.as_ref());
        match buildkit.cleanup_container(c).await {
            Ok(()) => println!("removed container {name}"),
            Err(e) => {
                eprintln!("error: failed to remove container {name}: {e}");
                failed = failed.saturating_add(1);
            },
        }
    }
    for i in &images {
        let name = SCellImageInfo::image_name(&i.id);
        match buildkit.cleanup_image(i).await {
            Ok(()) => println!("removed image {name}"),
            Err(e) => {
                eprintln!("error: failed to remove image {name}: {e}");
                failed = failed.saturating_add(1);
            },
        }
    }
    println!(
        "removed {} container(s), {} image(s), {} failure(s)",
        containers.len(),
        images.len(),
        failed
    );
    if failed > 0 {
        color_eyre::eyre::bail!("{failed} cleanup operation(s) failed");
    }
    Ok(())
}
