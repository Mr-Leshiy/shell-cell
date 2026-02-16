pub mod errors;
#[cfg(test)]
mod tests;

use std::{collections::HashSet, path::Path};

use color_eyre::eyre::{Context, ContextCompat};

use crate::{
    error::{OptionUserError, Report, UserError, WrapUserError},
    scell::{
        Link, SCell,
        compile::errors::{
            CircularTargets, DirNotFoundFromStmt, FileLoadFromStmt, MissingEntrypoint,
            MissingHangStmt, MissingShellStmt, MissingTarget, MountHostDirNotFound,
        },
        types::{
            SCellFile,
            name::TargetName,
            target::{config::ConfigStmt, from::FromStmt},
        },
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

        let entry_point = scell_f
            .cells
            .remove(&entry_point_target)
            .user_err(MissingEntrypoint(
                scell_f.location.clone(),
                entry_point_target.clone(),
            ))?;

        // Store processed target's name and location, to detect circular target dependencies
        let mut visited_targets = HashSet::new();

        let mut links = Vec::new();

        let mut walk_f = scell_f;
        let mut walk_target = entry_point;
        let mut walk_target_name = entry_point_target;
        let mut shell = None;
        let mut hang = None;
        let mut config = None;
        loop {
            // Use only the most recent 'shell` and 'hang' statements from the targets graph.
            if shell.is_none() {
                shell = walk_target.shell;
            }
            if hang.is_none() {
                hang = walk_target.hang;
            }
            if config.is_none() {
                config = resolve_config(&walk_f.location, &walk_target_name, walk_target.config)?;
            }
            links.push(Link::Node {
                name: walk_target_name.clone(),
                location: walk_f.location.clone(),
                workspace: walk_target.workspace.clone(),
                copy: walk_target.copy.clone(),
                build: walk_target.build.clone(),
                env: walk_target.env.clone(),
            });

            match walk_target.from {
                FromStmt::Image(docker_image_def) => {
                    links.push(Link::Root(docker_image_def));
                    break;
                },
                FromStmt::TargetRef { location, name } => {
                    if let Some(location) = location {
                        let location = walk_f.location.join(location);
                        let location =
                            std::fs::canonicalize(&location).user_err(DirNotFoundFromStmt(
                                location.clone(),
                                name.clone(),
                                walk_f.location.clone(),
                            ))?;
                        walk_f =
                            SCellFile::from_path(&location).wrap_user_err(FileLoadFromStmt(
                                location.clone(),
                                name.clone(),
                                walk_f.location.clone(),
                            ))?;
                    }

                    if visited_targets.contains(&(name.clone(), walk_f.location.clone())) {
                        return UserError::bail(CircularTargets(name.clone(), walk_f.location))?;
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

        let mut report = Report::new();
        if shell.is_none() {
            report.add_error(UserError::wrap(MissingShellStmt));
        }
        if hang.is_none() {
            report.add_error(UserError::wrap(MissingHangStmt));
        }
        report.check()?;

        Ok(Self(super::SCellInner {
            links,
            shell: shell.context("'shell' cannot be 'None'")?,
            hang: hang.context("'hang' cannot be 'None'")?,
            config,
        }))
    }
}

fn resolve_config(
    location: &Path,
    target_name: &TargetName,
    config: Option<ConfigStmt>,
) -> color_eyre::Result<Option<ConfigStmt>> {
    config
        .map(|mut c| {
            // resolve mounts
            c.mounts.0 = c
                .mounts
                .0
                .into_iter()
                .map(|mut m| {
                    m.host = std::fs::canonicalize(location.join(&m.host)).user_err(
                        MountHostDirNotFound(m.host, target_name.clone(), location.to_path_buf()),
                    )?;
                    color_eyre::eyre::Ok(m)
                })
                .collect::<Result<_, _>>()?;

            color_eyre::eyre::Ok(c)
        })
        .transpose()
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
