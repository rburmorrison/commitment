use std::{fs::File, io::Write, os::unix::prelude::PermissionsExt, path::Path};

use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("a pre-commit hook already exists")]
    HookExists,

    #[error("not a git repository")]
    NotGit,

    #[error("no commitment file")]
    NoCommitment,
}

#[derive(Clone, Copy, clap::Args)]
pub struct Args {
    #[arg(short, long)]
    /// Overwrite the existing pre-commit hook if one exists
    force: bool,
}

fn install_script(path: &Path) -> Result<()> {
    let mut f = File::create(path)?;
    writeln!(f, "#!/bin/env sh")?;
    writeln!(f)?;
    writeln!(f, "commitment execute commitment.yml")?;

    let mut permissions = f.metadata()?.permissions();
    permissions.set_mode(0o755);

    std::fs::set_permissions(path, permissions)?;

    Ok(())
}

pub fn execute(args: Args) -> Result<()> {
    let cwd = std::env::current_dir()?;

    let commitment_path = cwd.join("commitment.yml");
    if !commitment_path.exists() {
        bail!(Error::NoCommitment);
    }

    let git_path = cwd.join(".git");
    if !git_path.exists() {
        bail!(Error::NotGit);
    }

    let pre_commit_path = cwd.join(".git").join("hooks").join("pre-commit");
    if pre_commit_path.exists() && !args.force {
        bail!(Error::HookExists);
    }

    install_script(pre_commit_path.as_path())?;

    Ok(())
}
