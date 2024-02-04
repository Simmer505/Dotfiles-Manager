use std::error::Error;
use clap::ArgMatches;

use crate::config::config::Config;

pub mod config;
pub mod dotfile;
pub mod args;


pub fn run(args: ArgMatches, config: Config) -> Result<(), Box<dyn Error>> {

    let copy_to_sys = args.get_flag("from-git");

    let dotfiles = config.dotfiles;

    let valid_dotfiles: Vec<_> = dotfiles.iter().filter_map(|dotfile| match dotfile {
        Ok(dotfile) => Some(dotfile),
        Err(e) => {
            println!("Failed to read a dotfile: {}", e);
            None
        },
    }).collect();

    let copy_results = valid_dotfiles.iter().map(|dotfile| (dotfile.copy_dotfile(copy_to_sys), dotfile));

    copy_results.for_each(|result| {
        match result.0 {
            Err(e) => println!("Failed to copy dotfile: {}", e),
            _ => (),
        }
    });



    Ok(())
}
