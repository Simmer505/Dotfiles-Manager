use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    pub manager: Option<PathBuf>,

    #[arg(short, long, default_value_t=false)]
    pub from: bool,

    #[arg(short, long, default_value_t=false)]
    pub dry: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
