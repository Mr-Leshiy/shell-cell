//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. https://docs.docker.com/build/building/base-images/.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

use std::path::PathBuf;

use anyhow::Context;

use crate::scell_file::{SCellFile, def::FromStmt, docker::DockerImageDef, name::SCellName};

const SCELL_ENTRY_POINT: &str = "main";

#[derive(Debug)]
pub enum Link {
    Root(DockerImageDef),
    Node {
        name: SCellName,
        path: PathBuf,
        commands: Vec<String>,
    },
}

pub struct SCell(Vec<Link>);

impl SCell {
    /// Process the provided `SCellFile` file recursively, to build a proper chain of
    /// links for the Shell-Cell definition.
    pub fn build(
        mut scell_f: SCellFile,
        scell_path: PathBuf,
        entry: Option<SCellName>,
    ) -> anyhow::Result<Self> {
        let entry_point_name = entry.map(|e| Ok(e)).unwrap_or_else(|| {
            SCELL_ENTRY_POINT.parse().context(format!(
                "'{SCELL_ENTRY_POINT}' must be a valid Shell-Cell name"
            ))
        })?;

        let entry_point = scell_f.cells.remove(&entry_point_name).context(format!(
            "{} does not contain an entrypoint '{entry_point_name}'",
            scell_path.display()
        ))?;

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
                FromStmt::DockerImage(docker_image_def) => {
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

        Ok(Self(links))
    }
}
