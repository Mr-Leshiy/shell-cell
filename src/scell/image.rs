use std::{
    fmt::Write,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Context, ContextCompat};

use super::{
    Link, SCell,
    parser::{
        name::TargetName,
        target::{build::BuildStmt, copy::CopyStmt, workspace::WorkspaceStmt},
    },
};

impl SCell {
    /// Calculates a fast, non-cryptographic 'metrohash' hash value.
    /// Returns a hex string value.
    pub fn hex_hash(&self) -> String {
        let mut hasher = metrohash::MetroHash64::new();

        for link in &self.links {
            link.hash(&mut hasher);
        }
        self.shell.hash(&mut hasher);
        self.hang.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn prepare_image_tar_artifact(&self) -> color_eyre::Result<(tar::Builder<Vec<u8>>, &str)> {
        const DOCKERFILE_NAME: &str = "Dockerfile";
        // Unix file mode,
        // 6 (Owner): Read (4) + Write (2) = Read & Write.
        const FILE_MODE: u32 = 0o600;

        let mut tar = tar::Builder::new(Vec::new());
        let mut dockerfile = String::new();

        let mut links_iter = self.links.iter().rev().peekable();

        while let Some(link) = links_iter.next() {
            match link {
                Link::Root(root) => {
                    let _ = writeln!(dockerfile, "FROM {root}");
                },
                Link::Node {
                    build,
                    copy,
                    location: path,
                    workspace,
                    name,
                } => {
                    prepare_workspace_stmt(&mut dockerfile, workspace);
                    prepare_copy_stmt(&mut dockerfile, &mut tar, copy, path)?;
                    prepare_build_stmt(&mut dockerfile, build);
                    // The last item
                    if links_iter.peek().is_none() {
                        // Adding metadata
                        prepare_metadata_stmt(&mut dockerfile, name, path)?;
                    }
                },
            }
        }
        // TODO: find better solution how to hang the container
        prepare_hang_stmt(&mut dockerfile, &self.hang);

        // Attach generated dockerfile string to tar
        let mut header = tar::Header::new_gnu();
        header.set_path(DOCKERFILE_NAME)?;
        header.set_size(dockerfile.len() as u64);
        header.set_mode(FILE_MODE);
        header.set_cksum();
        tar.append(&header, dockerfile.as_bytes())?;

        Ok((tar, DOCKERFILE_NAME))
    }
}

/// Following Docker's `COPY` syntax, the last element in each
/// sequence is treated as the **destination**
/// inside the container and is excluded.
///
/// All other elements are treated as **source paths** and are joined
/// with the `ctx_path` to create absolute or relative paths rooted in the build
/// context.
fn prepare_copy_stmt<W: std::io::Write>(
    dockerfile: &mut String,
    tar: &mut tar::Builder<W>,
    copy_stmt: &CopyStmt,
    ctx_path: &Path,
) -> color_eyre::Result<()> {
    for e in &copy_stmt.0 {
        let mut iter = e.iter().peekable();
        let mut cp_tmt = String::new();
        while let Some(item) = iter.next() {
            // The last item is the destination to where to copy
            // https://docs.docker.com/reference/dockerfile/#copy
            if iter.peek().is_none() {
                let _ = write!(&mut cp_tmt, " {}", item.display());
            } else {
                // Tweaking the original item path from the `CopyStmt` by resolving
                // it with the corresponding
                // Shell-Cell blueprint file location where
                // `CopyStmt` locates. Making a path
                // a relative from the root
                // e.g. '/some/path/from/root' transforms to 'some/path/from/root'.
                // Copying files into the tar archive.
                let ctx_path = ctx_path
                    .parent()
                    .context("Must have a parent as its a path to the file")?;
                let item = ctx_path.join(item);
                let mut f = std::fs::File::open(&item)
                    .context(format!("Cannot open file {}", item.display()))?;
                let item: PathBuf = std::fs::canonicalize(item)?
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
        }

        let _ = writeln!(dockerfile, "COPY {cp_tmt}",);
    }
    Ok(())
}

fn prepare_metadata_stmt(
    dockerfile: &mut String,
    name: &TargetName,
    location: &Path,
) -> color_eyre::Result<()> {
    let _ = writeln!(dockerfile, "LABEL scell-name=\"{name}\"");
    let _ = writeln!(
        dockerfile,
        "LABEL scell-location=\"{}\"",
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

fn prepare_hang_stmt(
    dockerfile: &mut String,
    hang_stmt: &str,
) {
    let _ = writeln!(dockerfile, "ENTRYPOINT {hang_stmt}");
}
