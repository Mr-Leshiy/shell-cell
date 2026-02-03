use std::path::PathBuf;

use anyhow::Context;

use super::{Link, SCell};
use crate::scell_file::{SCellFile, name::SCellName, scell::FromStmt};

const SCELL_DEFAULT_ENTRY_POINT: &str = "main";

impl SCell {
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
                workspace: scell_walk_def.workspace.clone(),
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
}
