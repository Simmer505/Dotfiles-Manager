use std::path::PathBuf;

use dotfiles_manager::args::args;
use dotfiles_manager::config::config::Config;


fn main() -> Result<(), dotfiles_manager::ManagerError> {
    let args = args::parse_args();

    let program_config = Config::parse(PathBuf::from("/home/eesim/.config/dotfiles/config"))?;

    dotfiles_manager::run(args, program_config)

}

