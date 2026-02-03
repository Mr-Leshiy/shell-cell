//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

mod link;

use std::{
    fmt::Write,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use anyhow::Context;
use itertools::Itertools;

use crate::{
    scell::link::Link,
    scell_file::{SCellFile, name::SCellName, scell::FromStmt, shell::ShellStmt},
};

const SCELL_DEFAULT_ENTRY_POINT: &str = "main";

#[derive(Debug)]
pub struct SCell {
    links: Vec<Link>,
    shell: ShellStmt,
    hang: String,
}

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.shell.bin_path
    }

    /// Process the provided `SCellFile` file recursively, to build a proper chain of
    /// links for the Shell-Cell definition.
    pub fn compile(
        mut scell_f: SCellFile,
        scell_path: PathBuf,
        entry: Option<SCellName>,
    ) -> anyhow::Result<Self> {
        let entry_point_name = entry.map_or_else(
            || {
                SCELL_DEFAULT_ENTRY_POINT.parse().context(format!(
                    "'{SCELL_DEFAULT_ENTRY_POINT}' must be a valid Shell-Cell name"
                ))
            },
            Ok,
        )?;

        let entry_point = scell_f.cells.remove(&entry_point_name).context(format!(
            "{} does not contain an entrypoint '{entry_point_name}'",
            scell_path.display()
        ))?;

        let Some(shell) = entry_point.shell.clone() else {
            anyhow::bail!(
                "{}+{entry_point_name} endpoint does not contain 'shell' statement",
                scell_path.display()
            );
        };
        let Some(hang) = entry_point.hang.clone() else {
            anyhow::bail!(
                "{}+{entry_point_name} endpoint does not contain 'hang' statement",
                scell_path.display()
            );
        };

        let mut links = Vec::new();

        let mut scell_walk_f = scell_f;
        let mut scell_walk_def = entry_point;
        let mut scell_walk_name = entry_point_name;
        let mut scell_walk_path = scell_path;
        loop {
            links.push(Link::Node {
                name: scell_walk_name.clone(),
                path: scell_walk_path.clone(),
                copy: scell_walk_def.copy.clone(),
                build: scell_walk_def.build.clone(),
            });

            match scell_walk_def.from {
                FromStmt::Image(docker_image_def) => {
                    links.push(Link::Root(docker_image_def));
                    break;
                },
                FromStmt::SCellRef {
                    scell_path,
                    scell_def_name,
                } => {
                    if let Some(scell_path) = scell_path {
                        scell_walk_f = SCellFile::from_path(&scell_path)?;
                        scell_walk_path = scell_path;
                    }
                    scell_walk_def =
                        scell_walk_f.cells.remove(&scell_def_name).context(format!(
                            "{} does not contain a '{scell_def_name}'",
                            scell_walk_path.display()
                        ))?;
                    scell_walk_name = scell_def_name;
                },
            }
        }

        Ok(Self { links, shell, hang })
    }

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

    pub fn prepare_image_tar_artifact(&self) -> anyhow::Result<(tar::Builder<Vec<u8>>, &str)> {
        const DOCKERFILE_NAME: &str = "Dockerfile";
        // Unix file mode,
        // 6 (Owner): Read (4) + Write (2) = Read & Write.
        const FILE_MODE: u32 = 0o600;

        let mut tar = tar::Builder::new(Vec::new());
        let mut dockerfile = String::new();

        for link in self.links.iter().rev() {
            match link {
                Link::Root(root) => {
                    let _ = writeln!(dockerfile, "FROM {root}");
                },
                Link::Node {
                    build,
                    copy,
                    path: ctx_path,
                    ..
                } => {
                    // Following Docker's `COPY` syntax, the last element in each
                    // sequence is treated as the **destination**
                    // inside the container and is excluded.
                    //
                    // All other elements are treated as **source paths** and are joined
                    // with the `ctx_path` to create absolute or relative paths rooted in the build
                    // context.
                    for e in &copy.0 {
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
                                // Shell-Cell source file location where
                                // `CopyStmt` locates. Making a path
                                // a relative from the root
                                // e.g. '/some/path/from/root' transforms to 'some/path/from/root'.
                                // Copying files into the tar archive.
                                let ctx_path = ctx_path
                                    .parent()
                                    .context("Must have a parent as its a path to the file")?;
                                let item = ctx_path.join(item);
                                let mut f = std::fs::File::open(&item)?;
                                let item: PathBuf = std::fs::canonicalize(item)?
                                    .components()
                                    .filter(|c| {
                                        !matches!(
                                            c,
                                            std::path::Component::Prefix(_)
                                                | std::path::Component::RootDir
                                        )
                                    })
                                    .collect();

                                tar.append_file(&item, &mut f)?;
                                let _ = write!(&mut cp_tmt, " {}", item.display());
                            }
                        }

                        let _ = writeln!(dockerfile, "COPY {cp_tmt}",);
                    }

                    for e in &build.0 {
                        let _ = writeln!(dockerfile, "RUN {e}");
                    }
                },
            }
        }

        // TODO: find better solution how to hang the container
        let _ = writeln!(
            dockerfile,
            "SHELL [\"{}\", {}]",
            self.shell.bin_path,
            self.shell
                .commands
                .iter()
                .map(|v| format!("\"{v}\""))
                .join(",")
        );
        let _ = writeln!(&mut dockerfile, "ENTRYPOINT {}", self.hang);

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
