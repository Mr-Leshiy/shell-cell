use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

use bytes::Bytes;
use color_eyre::eyre::Context;

use super::{
    Link, METADATA_LOCATION_KEY, METADATA_TARGET_KEY, SCell,
    types::{
        name::TargetName,
        target::{build::BuildStmt, copy::CopyStmt, workspace::WorkspaceStmt},
    },
};
use crate::scell::types::target::env::EnvStmt;

impl SCell {
    pub fn prepare_image_tar_artifact_bytes(&self) -> color_eyre::Result<(Bytes, &str)> {
        const DOCKERFILE_NAME: &str = "Dockerfile";
        // Unix file mode,
        // 6 (Owner): Read (4) + Write (2) = Read & Write.
        const FILE_MODE: u32 = 0o600;

        let mut tar = tar::Builder::new(Vec::new());
        let mut dockerfile = String::new();

        let mut links_iter = self.0.links.iter().rev().peekable();

        while let Some(link) = links_iter.next() {
            match link {
                Link::Root(root) => {
                    let _ = writeln!(dockerfile, "FROM {root}");
                },
                Link::Node {
                    build,
                    copy,
                    location,
                    workspace,
                    env,
                    name,
                } => {
                    prepare_workspace_stmt(&mut dockerfile, workspace);
                    prepare_env_stmt(&mut dockerfile, env);
                    prepare_copy_stmt(&mut dockerfile, &mut tar, copy)?;
                    prepare_build_stmt(&mut dockerfile, build);
                    // The last item
                    if links_iter.peek().is_none() {
                        // Adding metadata
                        prepare_metadata_stmt(&mut dockerfile, name, location)?;
                    }
                },
            }
        }
        // TODO: find better solution how to hang the container
        prepare_hang_stmt(&mut dockerfile, &self.0.hang);

        // Attach generated dockerfile string to tar
        let mut header = tar::Header::new_gnu();
        header.set_path(DOCKERFILE_NAME)?;
        header.set_size(dockerfile.len() as u64);
        header.set_mode(FILE_MODE);
        header.set_cksum();
        tar.append(&header, dockerfile.as_bytes())?;

        Ok((tar.into_inner()?.into(), DOCKERFILE_NAME))
    }
}

/// Following Docker's `COPY` syntax, the last element in each
/// sequence is treated as the **destination**
/// inside the container and is excluded.
///
/// Its assumed that all **source paths** are absolute paths.
fn prepare_copy_stmt<W: std::io::Write>(
    dockerfile: &mut String,
    tar: &mut tar::Builder<W>,
    copy_stmt: &CopyStmt,
) -> color_eyre::Result<()> {
    for e in &copy_stmt.0 {
        let mut cp_tmt = String::new();
        for src_item in &e.src {
            // Tweaking the original item path from the `CopyStmt.
            // Making a path a relative from the root
            // e.g. '/some/path/from/root' transforms to 'some/path/from/root'.
            let mut f = std::fs::File::open(src_item)
                .context(format!("Cannot open file {}", src_item.display()))?;
            let item: PathBuf = src_item
                .components()
                .filter(|c| {
                    !matches!(
                        c,
                        std::path::Component::Prefix(_) | std::path::Component::RootDir
                    )
                })
                .collect();

            tar.append_file(&item, &mut f)?;
            let _ = write!(&mut cp_tmt, " {}", item.display());
        }

        let _ = write!(&mut cp_tmt, " {}", e.dest.display());

        let _ = writeln!(dockerfile, "COPY {cp_tmt}",);
    }
    Ok(())
}

fn prepare_metadata_stmt(
    dockerfile: &mut String,
    name: &TargetName,
    location: &Path,
) -> color_eyre::Result<()> {
    let _ = writeln!(dockerfile, "LABEL {METADATA_TARGET_KEY}=\"{name}\"");
    let _ = writeln!(
        dockerfile,
        "LABEL {METADATA_LOCATION_KEY}=\"{}\"",
        std::fs::canonicalize(location)?.display()
    );
    Ok(())
}

fn prepare_build_stmt(
    dockerfile: &mut String,
    build_stm: &BuildStmt,
) {
    for e in &build_stm.0 {
        let _ = writeln!(dockerfile, "RUN {e}");
    }
}

fn prepare_workspace_stmt(
    dockerfile: &mut String,
    workspace_stmt: &WorkspaceStmt,
) {
    if let Some(workspace) = &workspace_stmt.0 {
        let _ = writeln!(dockerfile, "WORKDIR {workspace}");
    }
}

fn prepare_env_stmt(
    dockerfile: &mut String,
    evn_stmt: &EnvStmt,
) {
    for e in &evn_stmt.0 {
        let _ = writeln!(dockerfile, "ENV {}={}", e.key, e.value);
    }
}

fn prepare_hang_stmt(
    dockerfile: &mut String,
    hang_stmt: &str,
) {
    let _ = writeln!(dockerfile, "ENTRYPOINT {hang_stmt}");
}
