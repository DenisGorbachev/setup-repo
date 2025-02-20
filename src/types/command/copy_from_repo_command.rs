use crate::Outcome;
use clap::{value_parser, Parser};
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
        // let Self {
        //     repo,
        //     target,
        //     paths
        // } = self;
        // AI ensure target is a directory
        // AI let repo_dir = // create temporary directory using `tempfile`
        // AI clone `repo` into `repo_dir`
        // AI for file in files copy them into `target`
        todo!()
    }
}
