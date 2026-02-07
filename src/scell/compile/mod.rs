pub mod errors;
#[cfg(test)]
mod tests;

use std::{collections::HashSet, path::Path};

use color_eyre::eyre::Context;

use super::{
    Link, SCell,
    parser::{SCellFile, name::TargetName, target::FromStmt},
};
use crate::{
    error::{OptionUserError, UserError, WrapUserError},
    scell::compile::errors::{
        CircularTargets, DirNotFoundFromStmt, FileLoadFromStmt, MissingTarget,
    },
    scell_home_dir,
};

const SCELL_DEFAULT_ENTRY_POINT: &str = "main";

impl SCell {
    /// Process the provided `SCellFile` file recursively, to build a proper chain of
    /// links for the Shell-Cell definition.
    pub fn compile<P: AsRef<Path>>(
        path: P,
        entry: Option<TargetName>,
    ) -> color_eyre::Result<Self> {
        let mut scell_f = SCellFile::from_path(path)?;
        let entry_point_target = entry.map_or_else(
            || {
                SCELL_DEFAULT_ENTRY_POINT.parse().context(format!(
                    "'{SCELL_DEFAULT_ENTRY_POINT}' must be a valid Shell-Cell name"
                ))
            },
            Ok,
        )?;

        let entry_point = scell_f.cells.remove(&entry_point_target).user_err(format!(
            "Shell-Cell file '{}' does not contain an entrypoint target '{entry_point_target}'",
            scell_f.location.display()
        ))?;

        // TODO: do not early return, return as much errors as possible
        // TODO: add proper error types as its done with `MissingTarget`, `FileLoadFromStmt` etc.
        let Some(shell) = entry_point.shell.clone() else {
            return UserError::bail(format!(
                "entrypoint target '{entry_point_target}' in '{}' does not contain 'shell' statement",
                scell_f.location.display()
            ))?;
        };
        let Some(hang) = entry_point.hang.clone() else {
            return UserError::bail(format!(
                "entrypoint target '{entry_point_target}' in '{}' does not contain 'hang' statement",
                scell_f.location.display()
            ))?;
        };

        // Store processed target's name and location, to detect circular target dependencies
        let mut visited_targets = HashSet::new();

        let mut links = Vec::new();

        let mut walk_f = scell_f;
        let mut walk_target = entry_point;
        let mut walk_target_name = entry_point_target;
        loop {
            links.push(Link::Node {
                name: walk_target_name.clone(),
                location: walk_f.location.clone(),
                workspace: walk_target.workspace.clone(),
                copy: walk_target.copy.clone(),
                build: walk_target.build.clone(),
            });

            match walk_target.from {
                FromStmt::Image(docker_image_def) => {
                    links.push(Link::Root(docker_image_def));
                    break;
                },
                FromStmt::TargetRef { location, name } => {
                    let current_target_location = walk_f.location.clone();
                    if let Some(location) = location {
                        let location = current_target_location.join(location);
                        let location =
                            std::fs::canonicalize(&location).user_err(DirNotFoundFromStmt(
                                location.clone(),
                                name.clone(),
                                current_target_location.clone(),
                            ))?;
                        walk_f =
                            SCellFile::from_path(&location).wrap_user_err(FileLoadFromStmt(
                                location.clone(),
                                name.clone(),
                                current_target_location.clone(),
                            ))?;
                    }

                    if visited_targets.contains(&(name.clone(), current_target_location.clone())) {
                        return UserError::bail(CircularTargets(
                            name.clone(),
                            current_target_location,
                        ))?;
                    }

                    walk_target = walk_f
                        .cells
                        .remove(&name)
                        .user_err(MissingTarget(name.clone(), walk_f.location.clone()))?;
                    walk_target_name = name;

                    visited_targets.insert((walk_target_name.clone(), walk_f.location.clone()));
                },
            }
        }

        Ok(Self { links, shell, hang })
    }
}

fn global() -> color_eyre::Result<Option<SCellFile>> {
    const SCELL_GLOBAL: &str = "global.yml";
    let scell_home = scell_home_dir()?;
    SCellFile::from_path(scell_home.join(SCELL_GLOBAL))
        .map(Some)
        .or_else(|e| {
            let io_e = e.downcast::<std::io::Error>()?;
            if io_e.kind() == std::io::ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(io_e.into())
            }
        })
}
