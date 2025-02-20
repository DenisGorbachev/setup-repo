use clap::Parser;
use setup_repo::{Cli, Outcome};
use std::io::{stderr, stdout};

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    let mut stdout = stdout();
    let mut stderr = stderr();
    args.run(&mut stdout, &mut stderr).await
}
