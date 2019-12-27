use super::data_model::*;
use std::path::PathBuf;
//use structopt::clap::Shell;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(flatten)]
    pub filters: Filters,
    #[structopt(flatten)]
    pub optional: Optional,

    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}
#[derive(StructOpt, Debug)]
pub struct Filters {
    #[structopt(short, long, default_value = "https,http,rsync")]
    pub protocols: Protocols,
    #[structopt(short, long)]
    pub country: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct Optional {
    #[structopt(long, default_value = "/etc/pacman.d/mirrorlist", parse(from_os_str))]
    pub save: PathBuf,
}

pub fn initialize() -> Cli {
//   let mut clap = Cli::clap();

//    clap.gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");
//    clap.gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, "target");

    Cli::from_args()
}
