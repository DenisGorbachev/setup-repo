use crate::Outcome;
use clap::Parser;
use std::io::Write;
use Command::*;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Print(CopyFromRepoCommand),
}

impl Command {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        match self {
            Print(command) => command.run(stdout, stderr).await,
        }
    }
}

mod copy_from_repo_command;

pub use copy_from_repo_command::*;
