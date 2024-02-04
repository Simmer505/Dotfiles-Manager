use std::path::PathBuf;

use dotfiles_manager::args::parse;
use dotfiles_manager::config::cfg;


fn main() -> Result<(), dotfiles_manager::ManagerError> {
    let args = parse::parse_args();

    let program_config = cfg::Config::parse(PathBuf::from("/home/eesim/.config/dotfiles/config"))?;

    dotfiles_manager::run(args, program_config)

}

