use std::error::Error;
use std::fmt;

use itertools::{Itertools, Either};

use crate::config::cfg;
use crate::dotfile::dot;
use crate::args::arg;

pub mod config;
pub mod dotfile;
pub mod args;
pub mod fs;




pub fn run(args: arg::Cli, config: cfg::Config) -> Result<(), ManagerError> {

    let copy_to_sys = args.from;

    let _dry_run = args.dry;

    let dotfiles = config.dotfiles;

    let (valid, unrecoverable_errors): (Vec<_>, Vec<_>) = dotfiles.into_iter().partition_result();

    if unrecoverable_errors.len() > 0 {
        for error in unrecoverable_errors.into_iter() {
            eprintln!("{:#?}", error);
            return Err(ManagerError::DotfileCreateError)
        }
    }


    let (error_free, contains_errors): (Vec<_>, Vec<_>) = valid
        .into_iter()
        .partition_map(
        |dotfile| 
            match dotfile.get_dir_errors() {
                errors if errors.is_empty() => Either::Left(dotfile),
                _ => Either::Right(dotfile),
            }
    );

    if contains_errors.len() > 0 {
        log_errored_dotfiles(&contains_errors).expect("Dotfile path is invalid"); 
    }

    let copy_results = error_free
        .iter()
        .map(|dotfile| dotfile.copy_dotfile(copy_to_sys));


    for result in copy_results {
        match result {
            Err(e) => println!("Failed to copy dotfile: {:?}", e),
            _ => (),
        }
    }



    Ok(())
}

fn log_errored_dotfiles(errors: &Vec<dot::ManagedDotfile>) -> Result<(), ManagerError> {

    for error in errors.into_iter() {

        if let dot::Dotfile::Dir(manager_dotfile) = &error.manager_dotfile {
            let dot_path = manager_dotfile.path.to_str().unwrap();
            println!("Error copying dotfile: {}", dot_path);
            manager_dotfile.errors
                .iter()
                .for_each(|error| println!("Error: {:?}", error));
        };

        if let dot::Dotfile::Dir(system_dotfile) = &error.system_dotfile {
            let Some(dot_path) = system_dotfile.path.to_str() else {
                return Err(ManagerError::DotfileInvalidPathError)
            };

            println!("Error copying dotfile: {}", dot_path);
            system_dotfile.errors
                .iter()
                .for_each(|error| println!("Error: {:?}", error));
        };

    }

    Ok(())
}




#[derive(Debug)]
pub enum ManagerError {
    DotfileCopyError(dot::DotfileError),
    ConfigParseError(cfg::ConfigParseError),
    DotfileCreateError,
    DotfileInvalidPathError,
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
            },
            ManagerError::DotfileCreateError => {
                write!(f, "Failed to read dotfiles")
            }
            ManagerError::DotfileInvalidPathError => {
                write!(f, "Dotfile has an invalid path")
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
