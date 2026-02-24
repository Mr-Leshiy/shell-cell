use std::{
    fmt::Write,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use bytes::Bytes;
use color_eyre::eyre::{Context, ContextCompat};
use dockerfile_parser_rs::{Dockerfile, Instruction};

use super::{
    Link,
    types::{
        name::TargetName,
        target::{build::BuildStmt, copy::CopyStmt, workspace::WorkspaceStmt},
    },
};
use crate::scell::{
    link::RootNode,
    types::target::{env::EnvStmt, hang::HangStmt},
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SCellImage {
    #[serde(flatten)]
    inner: SCellImageInner,
    #[serde(skip_serializing)]
    entry_point: TargetName,
    #[serde(skip_serializing)]
    blueprint_location: PathBuf,
    #[serde(skip_serializing)]
    dockerfile: Dockerfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
struct SCellImageInner {
    #[serde(rename = "targets-chain")]
    chain: Vec<Link>,
    hang: HangStmt,
}

impl SCellImage {
    pub fn entry_point(&self) -> &TargetName {
        &self.entry_point
    }

    pub fn location(&self) -> &Path {
        &self.blueprint_location
    }

    pub fn new(
        chain: Vec<Link>,
        hang: HangStmt,
    ) -> color_eyre::Result<Self> {
        let mut dockerfile_instructions = Vec::new();

        let inner = SCellImageInner { chain, hang };
        let mut links_iter = inner.chain.iter().rev().peekable();

        let mut entry_point = None;
        let mut blueprint_location = None;
        while let Some(link) = links_iter.next() {
            match link {
                Link::Root(RootNode::Image(image)) => {
                    dockerfile_instructions.push(Instruction::From {
                        platform: None,
                        image: image.to_string(),
                        alias: None,
                    });
                },
                Link::Root(RootNode::Dockerfile(docker_path)) => {
                    prepare_dockerfile(&mut dockerfile_instructions, docker_path)?;
                },
                Link::Node {
                    build,
                    copy,
                    location,
                    workspace,
                    env,
                    name,
                } => {
                    prepare_workspace_stmt(&mut dockerfile_instructions, workspace);
                    prepare_env_stmt(&mut dockerfile_instructions, env);
                    prepare_copy_stmt(&mut dockerfile_instructions, copy)?;
                    prepare_build_stmt(&mut dockerfile_instructions, build);
                    // The last item
                    if links_iter.peek().is_none() {
                        // Adding metadata
                        entry_point = Some(name.clone());
                        blueprint_location = Some(location.clone());
                    }
                },
            }
        }
        // TODO: find better solution how to hang the container
        prepare_hang_stmt(&mut dockerfile_instructions, &inner.hang);

        let dockerfile = Dockerfile::new(dockerfile_instructions);

        Ok(Self {
            inner,
            entry_point: entry_point.context("'entry_point' cannot be None")?,
            blueprint_location: blueprint_location
                .context("'blueprint_location' cannot be None")?,
            dockerfile,
        })
    }

    pub fn hash<H: Hasher>(
        &self,
        hasher: &mut H,
    ) -> color_eyre::Result<()> {
        self.inner.hash(hasher);
        self.dump_to_string()?.hash(hasher);
        Ok(())
    }

    fn dump_to_string(&self) -> color_eyre::Result<String> {
        let mut dockerfile_str = String::new();
        let mut iter = self.dockerfile.instructions.iter().peekable();
        while let Some(instruction) = iter.next() {
            if iter.peek().is_none() {
                if let Instruction::Entrypoint(entrypoint) = instruction
                    && let [entrypoint] = entrypoint.as_slice()
                {
                    writeln!(&mut dockerfile_str, "ENTRYPOINT {entrypoint}")?;
                } else {
                    color_eyre::eyre::bail!("Last instruction MUST be only single ETRYPOINT item");
                }
            } else {
                writeln!(&mut dockerfile_str, "{instruction}")?;
            }
        }
        Ok(dockerfile_str)
    }

    pub fn image_tar_artifact_bytes(&self) -> color_eyre::Result<(Bytes, &str)> {
        const DOCKERFILE_NAME: &str = "Dockerfile";
        const TEMP_DIR_PREFIX: &str = "scell";
        // Unix file mode,
        // 6 (Owner): Read (4) + Write (2) = Read & Write.
        const FILE_MODE: u32 = 0o600;

        let mut tar = tar::Builder::new(Vec::new());
        for i in &self.dockerfile.instructions {
            match i {
                Instruction::Copy { sources, .. } | Instruction::Add { sources, .. } => {
                    for s in sources {
                        let s = Path::new(s);
                        color_eyre::eyre::ensure!(
                            s.is_absolute() && s.exists(),
                            "Must be an absolute path and exists"
                        );
                        // Tweaking the original item path
                        // Making a path a relative from the root
                        // e.g. '/some/path/from/root' transforms to 'some/path/from/root'.
                        let item: PathBuf = s
                            .components()
                            .filter(|c| {
                                !matches!(
                                    c,
                                    std::path::Component::Prefix(_) | std::path::Component::RootDir
                                )
                            })
                            .collect();

                        if s.is_file() {
                            let mut f = std::fs::File::open(s)
                                .context(format!("Cannot open file {}", s.display()))?;
                            tar.append_file(&item, &mut f)?;
                        }
                        if s.is_dir() {
                            tar.append_dir_all(&item, s)?;
                        }
                    }
                },
                _ => {},
            }
        }

        let dockerfile_str = self.dump_to_string()?;
        // Attach generated dockerfile string to tar
        let mut header = tar::Header::new_gnu();
        header.set_path(DOCKERFILE_NAME)?;
        header.set_size(dockerfile_str.len() as u64);
        header.set_mode(FILE_MODE);
        header.set_cksum();
        tar.append(&header, dockerfile_str.as_bytes())?;
        Ok((tar.into_inner()?.into(), DOCKERFILE_NAME))
    }
}

/// Following Docker's `COPY` syntax, the last element in each
/// sequence is treated as the **destination**
/// inside the container and is excluded.
fn prepare_copy_stmt(
    dockerfile_instructions: &mut Vec<Instruction>,
    copy_stmt: &CopyStmt,
) -> color_eyre::Result<()> {
    for e in &copy_stmt.0 {
        dockerfile_instructions.push(Instruction::Copy {
            from: None,
            chown: None,
            chmod: None,
            link: None,
            sources: e
                .src
                .iter()
                .map(|s| {
                    color_eyre::eyre::ensure!(
                        s.is_absolute(),
                        "prepare_copy_stmt, path be absolute"
                    );
                    Ok(format!("{}", s.display()))
                })
                .collect::<Result<_, _>>()?,
            destination: format!("{}", e.dest.display()),
        });
    }
    Ok(())
}

fn prepare_dockerfile(
    dockerfile_instructions: &mut Vec<Instruction>,
    dockerfile_p: &Path,
) -> color_eyre::Result<()> {
    color_eyre::eyre::ensure!(
        dockerfile_p.is_absolute(),
        "prepare_dockerfile, path be absolute"
    );
    let mut dockerfile =
        Dockerfile::from(dockerfile_p.to_path_buf()).map_err(|e| color_eyre::eyre::eyre!(e))?;

    let dir = dockerfile_p
        .parent()
        .context("Dockerfile must have a parent directory")?;
    for i in &mut dockerfile.instructions {
        match i {
            Instruction::Copy { sources, .. } | Instruction::Add { sources, .. } => {
                for s in sources {
                    *s = format!("{}", dir.join(&s).display());
                }
            },
            _ => {},
        }
    }

    dockerfile_instructions.extend(dockerfile.instructions);
    Ok(())
}

fn prepare_build_stmt(
    dockerfile_instructions: &mut Vec<Instruction>,
    build_stm: &BuildStmt,
) {
    for e in &build_stm.0 {
        dockerfile_instructions.push(Instruction::Run {
            mount: None,
            network: None,
            security: None,
            command: vec![e.clone()],
            heredoc: None,
        });
    }
}

fn prepare_workspace_stmt(
    dockerfile_instructions: &mut Vec<Instruction>,
    workspace_stmt: &WorkspaceStmt,
) {
    if let Some(workspace) = &workspace_stmt.0 {
        dockerfile_instructions.push(Instruction::Workdir {
            path: workspace.clone(),
        });
    }
}

fn prepare_env_stmt(
    dockerfile_instructions: &mut Vec<Instruction>,
    evn_stmt: &EnvStmt,
) {
    if evn_stmt.0.is_empty() {
        return;
    }
    let env_inst = evn_stmt
        .0
        .iter()
        .cloned()
        .map(|e| (e.key, e.value))
        .collect();
    dockerfile_instructions.push(Instruction::Env(env_inst));
}

fn prepare_hang_stmt(
    dockerfile_instructions: &mut Vec<Instruction>,
    hang_stmt: &HangStmt,
) {
    dockerfile_instructions.push(Instruction::Entrypoint(vec![hang_stmt.0.clone()]));
}
