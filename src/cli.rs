use bitflags::bitflags;
use std::path::PathBuf;
use structopt::clap::Shell;
use structopt::StructOpt;

bitflags! {
    pub struct Protocols: u32 {
        const HTTP =  0b00000001;
        #[allow(non_upper_case_globals)]
        const HTTPS = 0b00000010;
    }
}

impl std::str::FromStr for Protocols {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Protocols::empty();
        let split: Vec<&str> = s.split(',').collect();
        if split.contains(&"https") {
            result = result | Protocols::HTTPS;
        }
        if split.contains(&"http") {
            result = result | Protocols::HTTP;
        }
        Ok(result)
    }
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(flatten)]
    pub filters: Filters,
    #[structopt(flatten)]
    pub optional: Optional,
}

#[derive(StructOpt, Debug)]
pub struct Filters {
    #[structopt(short, long, default_value = "http,https")]
    pub protocol: Protocols,
    #[structopt(short, long)]
    pub country: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct Optional {
    #[structopt(long, default_value = "/etc/pacman.d/mirrorlist", parse(from_os_str))]
    pub save: PathBuf,
}

pub fn initialize() -> Cli {
    let mut clap = Cli::clap();

    clap.gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");
    clap.gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, "target");

    Cli::from_args()
}
