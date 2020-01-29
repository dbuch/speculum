use crate::speculum::Protocols;
use anyhow::Result;
use log::LevelFilter;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(flatten)]
    pub filters: Filters,
    #[structopt(flatten)]
    pub optional: Optional,
    #[structopt(flatten)]
    pub logging: Logging,
}

#[derive(StructOpt, Debug)]
pub struct Logging {
    /// Increase verbosity (i.e. "-vvv" gives LogLevel::Debug)
    #[structopt(short, parse(from_occurrences))]
    pub verbosity: u8,
    /// Logging filter
    #[structopt(long, default_value = "speculum")]
    pub module_filter: String,
}

#[derive(StructOpt, Debug)]
pub struct Filters {
    /// Connection protocol
    #[structopt(long, default_value = "https,http")]
    pub protocols: Protocols,
    /// Country code (i.e. "en" or "us")
    #[structopt(long)]
    pub country: Option<String>,

    #[structopt(long, default_value = "30")]
    pub latest: usize,
}

#[derive(StructOpt, Debug)]
pub struct Optional {
    /// Writes the recieved mirrorlist in pacman format to file
    #[structopt(long, default_value = "/dev/stdout", parse(from_os_str))]
    pub write: PathBuf,
    /// The time before cache is invalidated (in secs [s])
    #[structopt(long, default_value = "300")]
    pub cache_timeout: u64,
    /// The time before connection is invalidated (in secs [s])
    #[structopt(long, default_value = "5")]
    pub connection_timeout: u64,
}

impl Cli {
    pub fn initialize() -> Result<Cli> {
        let cli = Cli::from_args();

        // Configure Logging
        {
            let mut logger = env_logger::builder();
            let level = match cli.logging.verbosity {
                0 => LevelFilter::Warn,
                1 => LevelFilter::Info,
                2 => LevelFilter::Debug,
                _ => LevelFilter::Trace,
            };

            logger.filter(Some(&cli.logging.module_filter), level);
            logger.try_init()?;
        }

        Ok(cli)
    }
}
