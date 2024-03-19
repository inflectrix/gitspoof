use std::{error::Error, path::PathBuf};

use clap::Parser;
use git2::{Repository, Signature};

#[derive(Parser)]
#[command(
    name = "gitspoof",
    about =  "Spoofs the author of a commit",
)]
struct Cli {
    #[clap(short, long, help = "The repo to add the spoofed commit to")]
    repo: PathBuf,

    #[clap(short, long, help = "The name of the author to spoof")]
    name: String,

    #[clap(short, long, help = "The email of the author to spoof")]
    email: String,

    #[clap(short, long, help = "The commit message")]
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let sig = Signature::now(&args.name, &args.email)?;

    let repo = Repository::open(args.repo)?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &args.message,
        &repo.head()?.peel_to_tree()?,
        &[],
    )?;

    Ok(())
}
