use std::path::Path;

use color_eyre::eyre::Context;

use super::{
    Link, SCell,
    parser::{SCellFile, name::TargetName, target::FromStmt},
};
use crate::{
    error::{OptionUserError, UserError, WrapUserError},
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
                    if let Some(location) = location {
                        walk_f = SCellFile::from_path(&location).user_err(format!(
                            "Fail to process 'from' statement for target '{name}' at '{}'",
                            location.display()
                        ))?;
                    }

                    walk_target = walk_f.cells.remove(&name).user_err(format!(
                        "Shell-Cell file '{}' does not contain a target '{name}'",
                        walk_f.location.display()
                    ))?;
                    walk_target_name = name;
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
