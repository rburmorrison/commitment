use std::{fs::File, io::Write, os::unix::prelude::PermissionsExt};

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

    let mut f = File::create(&pre_commit_path)?;
    writeln!(f, "#!/bin/env sh")?;
    writeln!(f)?;
    writeln!(f, "commitment execute commitment.yml")?;

    let mut permissions = f.metadata()?.permissions();
    permissions.set_mode(0o755);

    std::fs::set_permissions(pre_commit_path, permissions)?;

    Ok(())
}
