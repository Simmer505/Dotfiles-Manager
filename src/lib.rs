use std::error::Error;
use std::fmt;

use clap::ArgMatches;

use crate::config::cfg;
use crate::dotfile::dot;

pub mod config;
pub mod dotfile;
pub mod args;
pub mod fs;




pub fn run(args: ArgMatches, config: cfg::Config) -> Result<(), ManagerError> {

    let copy_to_sys = args.get_flag("from-git");

    let dotfiles = config.dotfiles;

    let valid_dotfiles: Vec<_> = dotfiles
        .iter()
        .filter_map(|dotfile| match dotfile {
            Ok(dotfile) => Some(dotfile),
            Err(e) => {
                eprintln!("Failed to read a dotfile: {:?}", e);
                None
            },
    }).collect();


    let errored_dotfiles = valid_dotfiles
        .iter()
        .filter_map(|dotfile| 
            match dotfile.get_dotfile_dir_errors() {
                errors if !errors.is_empty() => Some(dotfile),
                _ => None
            }
    );


    let _ = errored_dotfiles.map(|dotfile| {

        if let dot::Dotfile::Dir(manager_dotfile) = &dotfile.manager_dotfile {
            println!("Error copying dotfile: {}", manager_dotfile.path.to_str()?);
            manager_dotfile.errors
                .iter()
                .for_each(|error| println!("Error: {:?}", error));
        };

        if let dot::Dotfile::Dir(system_dotfile) = &dotfile.system_dotfile {
            println!("Error copying dotfile: {}", system_dotfile.path.to_str()?);
            system_dotfile.errors
                .iter()
                .for_each(|error| println!("Error: {:?}", error));
        };

        Some(())
    });

    let copy_results = valid_dotfiles.iter().map(|dotfile| dotfile.copy_dotfile(copy_to_sys));

    copy_results.for_each(|result| {
        match result {
            Err(e) => println!("Failed to copy dotfile: {:?}", e),
            _ => (),
        }
    });



    Ok(())
}




#[derive(Debug)]
pub enum ManagerError {
    DotfileCopyError(dot::DotfileError),
    ConfigParseError(cfg::ConfigParseError),
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

impl From<dot::DotfileError> for ManagerError {
    fn from(error: dot::DotfileError) -> ManagerError {
        ManagerError::DotfileCopyError(error)
    }
}

impl From<cfg::ConfigParseError> for ManagerError {
    fn from(error: cfg::ConfigParseError) -> ManagerError {
        ManagerError::ConfigParseError(error)
    }
}
