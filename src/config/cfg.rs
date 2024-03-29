use std::fs;
use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::fmt;

use toml::Table;

use crate::dotfile::dot::{self, ManagedDotfile};




pub struct Config {
    pub manager_dir: PathBuf,
    pub dotfiles: Vec<Result<ManagedDotfile, ConfigParseError>>,
}

impl Config {
    pub fn parse(path: PathBuf) -> Result<Self, ConfigParseError> {

        let config_file = Config::read_config(path)?;

        let dotfiles = Config::get_dotfiles(&config_file)?;

        let manager_dir = Config::get_manager_dir(&config_file)?;

        Ok(Config{manager_dir, dotfiles})
    }


    fn read_config(path: PathBuf) -> Result<Table, ConfigParseError> {

        let file = fs::read(path)?;

        let read_file = String::from_utf8(file)?;
        
        let config: Table = read_file.parse()?;

        Ok(config)

    }


    fn get_dotfiles(config: &Table) -> Result<Vec<Result<ManagedDotfile, ConfigParseError>>, ConfigParseError> {

        let read_dotfiles = config.get("dotfiles");
        
        let dotfiles = match read_dotfiles {
            Some(dotfiles) => dotfiles,
            None => return Err(ConfigParseError::DotfilesParseError),
        };

        let dotfile_iter = match dotfiles.as_array() {
            Some(dotfiles) => dotfiles.iter(), 
            None => return Err(ConfigParseError::DotfilesArrayParseError),
        };


        let dotfiles = dotfile_iter.map(|dotfile| {

                let dotfile_table = match dotfile.as_table() {
                    Some(table) => table,
                    None => return Err(ConfigParseError::DotfilesParseError),
                };

                let manager_path = PathBuf::from(
                    match dotfile_table.get("manager_path") {
                        Some(path) => path.as_str().expect("Invalid character in dotfile path"),
                        None => return Err(ConfigParseError::DotfilesTableParseError),
                    }
                );

                let system_path = PathBuf::from(
                    match dotfile_table.get("system_path") {
                        Some(path) => path.as_str().expect("Invalid character in dotfile path"),
                        None => return Err(ConfigParseError::DotfilesTableParseError),
                    }
                );

                Ok(ManagedDotfile::new(manager_path, system_path)?)
        });

        Ok(dotfiles.collect())
    }


    fn get_manager_dir(config: &Table) -> Result<PathBuf, ConfigParseError> {

        let home_dir = PathBuf::from(env::var("HOME")?);

        let manager_dir = if config.contains_key("manager_directory") {
            match config.get("manager_directory").unwrap().as_str() {
                Some(string) if PathBuf::from(string).is_absolute() => PathBuf::from(string),
                Some(string) => home_dir.join(PathBuf::from(string)),
                None => return Err(ConfigParseError::InvalidManagerDirectoryError),
            }
        } else {
            home_dir.join(PathBuf::from(".dotfiles"))
        };

        Ok(manager_dir)
    }

}




#[derive(Debug)]
pub enum ConfigParseError {
    FileReadError(std::io::Error),
    FromUtfError(std::string::FromUtf8Error),
    TomlParseError(toml::de::Error),
    ConfigEnvVariableError(std::env::VarError),
    DotfilesCreateError(dot::DotfileError),
    DotfilesParseError,
    DotfilesArrayParseError,
    DotfilesTableParseError,
    InvalidManagerDirectoryError,
}

impl Error for ConfigParseError {}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigParseError::FileReadError(io_error) => {
                write!(f, "{}", io_error)
            },
            ConfigParseError::FromUtfError(utf_error) => {
                write!(f, "{}", utf_error)
            },
            ConfigParseError::TomlParseError(parse_error) => {
                write!(f, "{}", parse_error)
            },
            ConfigParseError::DotfilesCreateError(create_error) => {
                write!(f, "{}", create_error)
            },
            ConfigParseError::ConfigEnvVariableError(env_error) => {
                write!(f, "{}", env_error)
            }
            ConfigParseError::DotfilesParseError => {
                write!(f, "Dotfiles section not found in config file")
            },
            ConfigParseError::DotfilesArrayParseError => {
                write!(f, "Dotfiles is not a valid array, Hint: use [[dotfiles]]")
            },
            ConfigParseError::DotfilesTableParseError => {
                write!(f, "Dotfile table is not valid")
            },
            ConfigParseError::InvalidManagerDirectoryError => {
                write!(f, "Manager directory setting in config is not valid")
            }
        }
    }
}

impl From<std::io::Error> for ConfigParseError {
    fn from(error: std::io::Error) -> ConfigParseError {
        ConfigParseError::FileReadError(error)
    }
}

impl From<std::string::FromUtf8Error> for ConfigParseError {
    fn from(error: std::string::FromUtf8Error) -> ConfigParseError {
        ConfigParseError::FromUtfError(error)
    }
}

impl From<std::env::VarError> for ConfigParseError {
    fn from(error: std::env::VarError) -> ConfigParseError {
        ConfigParseError::ConfigEnvVariableError(error) 
    }

}

impl From<toml::de::Error> for ConfigParseError {
    fn from(error: toml::de::Error) -> ConfigParseError {
        ConfigParseError::TomlParseError(error)
    }
}

impl From<dot::DotfileError> for ConfigParseError {
    fn from(error: dot::DotfileError) -> ConfigParseError {
        ConfigParseError::DotfilesCreateError(error)
    }
}
