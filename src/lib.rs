use std::error::Error;
use std::fmt;

use clap::ArgMatches;

use crate::config::config::Config;

pub mod config;
pub mod dotfile;
pub mod args;


pub fn run(args: ArgMatches, config: Config) -> Result<(), ManagerError> {

    let copy_to_sys = args.get_flag("from-git");

    let dotfiles = config.dotfiles;

    let valid_dotfiles: Vec<_> = dotfiles.iter().filter_map(|dotfile| match dotfile {
        Ok(dotfile) => Some(dotfile),
        Err(e) => {
            eprintln!("Failed to read a dotfile: {:?}", e);
            None
        },
    }).collect();

    let copy_results = valid_dotfiles.iter().map(|dotfile| (dotfile.copy_dotfile(copy_to_sys), dotfile));

    copy_results.for_each(|result| {
        match result.0 {
            Err(e) => println!("Failed to copy dotfile: {:?}", e),
            _ => (),
        }
    });



    Ok(())
}


#[derive(Debug)]
pub enum ManagerError {
    DotfileCopyError(dotfile::dotfile::DotfileError),
    ConfigParseError(config::config::ConfigParseError),
}

impl Error for ManagerError {}

impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManagerError::DotfileCopyError(copy_error) => {
                write!(f, "{}", copy_error)
            },
            ManagerError::ConfigParseError(parse_error) => {
                write!(f, "{}", parse_error)
            }
        }
    }
}

impl From<dotfile::dotfile::DotfileError> for ManagerError {
    fn from(error: dotfile::dotfile::DotfileError) -> ManagerError {
        ManagerError::DotfileCopyError(error)
    }
}

impl From<config::config::ConfigParseError> for ManagerError {
    fn from(error: config::config::ConfigParseError) -> ManagerError {
        ManagerError::ConfigParseError(error)
    }
}
