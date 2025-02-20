use crate::Outcome;
use clap::{value_parser, Parser};
use git2::Repository;
use helpful::bail;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Clone, Debug)]
pub struct CopyFromRepoCommand {
    /// Copy from this repo
    #[arg(short, long, value_parser = value_parser!(Url))]
    repo: Url,

    /// Copy into this directory
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    target: PathBuf,

    /// The relative paths to copy
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    paths: Vec<PathBuf>,
}

impl CopyFromRepoCommand {
    pub async fn run(self, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        let Self {
            repo,
            target,
            paths,
        } = self;

        if !target.try_exists()? {
            bail!("Target must exist: {}", target.display())
        }

        if !target.is_dir() {
            bail!("Target must be a directory: {}", target.display())
        }

        // Create temporary directory for cloning
        let temp_dir = tempfile::tempdir()?;

        // Clone repository
        Repository::clone(repo.as_str(), temp_dir.path())?;

        for path in paths {
            let source = temp_dir.path().join(&path);
            let dest = target.join(&path);

            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&source, &dest)?;
        }

        Ok(())
    }
}
