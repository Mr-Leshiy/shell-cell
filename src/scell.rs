//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

use std::{fmt::Write, hash::Hasher, path::PathBuf};

use anyhow::Context;
use itertools::Itertools;

use crate::scell_file::{
    SCellFile, def::FromStmt, image::ImageDef, name::SCellName, shell::ShellDef,
};

const SCELL_DEFAULT_ENTRY_POINT: &str = "main";

#[derive(Debug)]
pub enum Link {
    Root(ImageDef),
    Node {
        name: SCellName,
        path: PathBuf,
        commands: Vec<String>,
    },
}

#[derive(Debug)]
pub struct SCell {
    links: Vec<Link>,
    shell: ShellDef,
    hang: String,
}

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.shell.bin_path
    }

    /// Process the provided `SCellFile` file recursively, to build a proper chain of
    /// links for the Shell-Cell definition.
    pub fn build(
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
                commands: scell_walk_def.run.clone(),
            });

            match scell_walk_def.from {
                FromStmt::Image(docker_image_def) => {
                    links.push(Link::Root(docker_image_def));
                    break;
                },
                FromStmt::SCellDef {
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

        hasher.write(self.shell.bin_path.as_bytes());
        for v in &self.shell.commands {
            hasher.write(v.as_bytes());
        }
        for link in &self.links {
            match link {
                Link::Root(root) => hasher.write(format!("{root}").as_bytes()),
                Link::Node {
                    name,
                    path,
                    commands,
                } => {
                    hasher.write(format!("{name}").as_bytes());
                    hasher.write(format!("{}", path.display()).as_bytes());
                    for cmd in commands {
                        hasher.write(cmd.as_bytes());
                    }
                },
            }
        }
        format!("{:x}", hasher.finish())
    }

    /// Makes a Dockerfile for building an image
    pub fn to_dockerfile(&self) -> String {
        let mut dockerfile = String::new();
        for link in self.links.iter().rev() {
            match link {
                Link::Root(root) => {
                    let _ = writeln!(&mut dockerfile, "FROM {root}");
                },
                Link::Node { commands, .. } => {
                    for cmd in commands {
                        let _ = writeln!(&mut dockerfile, "RUN {cmd}");
                    }
                },
            }
        }
        let _ = writeln!(
            &mut dockerfile,
            "SHELL [\"{}\", {}]",
            self.shell.bin_path,
            self.shell
                .commands
                .iter()
                .map(|v| format!("\"{v}\""))
                .join(",")
        );
        // TODO: find better solution how to hang the container
        let _ = writeln!(&mut dockerfile, "ENTRYPOINT {}", self.hang);
        dockerfile
    }
}
