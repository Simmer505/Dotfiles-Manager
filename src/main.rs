use std::path::PathBuf;

use dotfiles_manager::args::arg;
use dotfiles_manager::config::cfg;




fn main() -> Result<(), dotfiles_manager::ManagerError> {

    let cmd = arg::parse_args();

    let program_config = cfg::Config::parse(PathBuf::from("/home/eesim/.config/dotfiles/config"))?;

    dotfiles_manager::run(cmd, program_config)

}
