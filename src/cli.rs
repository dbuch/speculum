use crate::speculum::{Protocols, Result};
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
    #[structopt(short, long, default_value = "https,http")]
    pub protocols: Protocols,
    #[structopt(short, long)]
    pub country: Option<String>,

    #[structopt(short, long, default_value = "30")]
    pub latest: usize,
}

#[derive(StructOpt, Debug)]
pub struct Optional {
    #[structopt(long, default_value = "/etc/pacman.d/mirrorlist", parse(from_os_str))]
    pub save: PathBuf,
}

impl Cli {
    pub fn initialize() -> Result<Cli> {
        let cli = Cli::from_args();
        let mut logger_builder = env_logger::builder();

        let level = match cli.verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Warn,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        };

        logger_builder.filter(Some("speculum"), level);
        logger_builder.try_init()?;

        //   let mut clap = Cli::clap();

        //    clap.gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");
        //    clap.gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, "target");

        Ok(cli)
    }
}
