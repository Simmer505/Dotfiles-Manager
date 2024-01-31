use std::path::PathBuf;

use dotfiles_manager::args;
use dotfiles_manager::program_config::ProgramConfig;


fn main() {
    let args = args::parse_args();

    let program_config = ProgramConfig::parse(PathBuf::from("/home/eesim/.config/dotfiles/config"));

    if let Err(e) = dotfiles_manager::run(args, program_config.unwrap()) {
        panic!("Error: {}", e)
    };


}
