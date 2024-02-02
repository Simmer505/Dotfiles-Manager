use std::path::PathBuf;

use dotfiles_manager::args;
use dotfiles_manager::config::Config;


fn main() {
    let args = args::parse_args();

    let program_config = Config::parse(PathBuf::from("/home/eesim/.config/dotfiles/config"));

    if let Err(e) = dotfiles_manager::run(args, program_config.unwrap()) {
        panic!("Error: {}", e)
    };


}
