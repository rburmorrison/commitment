use std::{
    fs::File,
    io::Write,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("a pre-commit hook already exists")]
    HookExists,

    #[error("failed to detect a git repository")]
    NotGit,

    #[error("no commitment file")]
    NoCommitment,

    #[error("commitment file is not within a git repository")]
    InvalidCommitmentLocation,
}

#[derive(clap::Args)]
pub struct Args {
    #[arg(name = "FILE", default_value = "commitment.yml")]
    /// Path to the commitment file to install
    config: PathBuf,

    #[arg(short, long)]
    /// Overwrite the existing pre-commit hook if one exists
    force: bool,
}

/// Returns the path to the closest directory that contains a `.git` directory.
fn closest_git_root(mut start: PathBuf) -> Option<PathBuf> {
    loop {
        let git_path = start.join(".git");

        if git_path.exists() {
            return Some(start);
        }

        start = match start.parent() {
            Some(parent) => parent.to_owned(),
            None => break,
        };
    }

    None
}

fn install_script(pre_commit_path: &Path, commitment_path: &Path) -> Result<()> {
    let commitment_path = commitment_path
        .display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('\"', "\\\"");

    let mut f = File::create(pre_commit_path)?;
    writeln!(f, "#!/bin/env sh")?;
    writeln!(f)?;
    writeln!(f, r#"commitment execute "{commitment_path}""#)?;

    let mut permissions = f.metadata()?.permissions();
    permissions.set_mode(0o755);

    std::fs::set_permissions(pre_commit_path, permissions)?;

    Ok(())
}

pub fn execute(args: &Args) -> Result<()> {
    let git_root = closest_git_root(std::env::current_dir()?).ok_or(Error::NotGit)?;

    let commitment_path = args.config.canonicalize()?;
    if !commitment_path.exists() {
        bail!(Error::NoCommitment);
    }

    let result = commitment_path.strip_prefix(&git_root);
    if result.is_err() {
        bail!(Error::InvalidCommitmentLocation);
    }

    let relative_path = result.unwrap();

    let pre_commit_path = git_root.join(".git").join("hooks").join("pre-commit");
    if pre_commit_path.exists() && !args.force {
        bail!(Error::HookExists);
    }

    install_script(&pre_commit_path, relative_path)?;

    Ok(())
}
