use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::fmt;

use toml::Table;

use crate::dotfile::Dotfile;


pub struct Config {
    pub manager_dir: PathBuf,
    pub dotfiles: Vec<Result<Dotfile, Box<dyn Error>>>,
}


impl Config {
    pub fn parse(path: PathBuf) -> Result<Self, Box<dyn Error>> {

        let config_file = Config::read_config(path)?;

        let dotfiles = Config::get_dotfiles(&config_file)?;

        let manager_dir = Config::get_manager_dir(&config_file);

        Ok(Config{manager_dir, dotfiles})
    }


    fn read_config(path: PathBuf) -> Result<Table, Box<dyn Error>> {

        let file = fs::read(path)?;

        let read_file = String::from_utf8(file)?;
        
        let config: Table = read_file.parse()?;

        Ok(config)

    }


    fn get_dotfiles(config: &Table) -> Result<Vec<Result<Dotfile, Box<dyn Error>>>, Box<dyn Error>> {

        let read_dotfiles = config.get("dotfiles");
        
        let dotfiles = match read_dotfiles {
            Some(dotfiles) => dotfiles,
            None => return Err(Config::produce_error(1)),
        };

        let dotfile_iter = match dotfiles.as_array() {
            Some(dotfiles) => dotfiles.iter(), 
            None => return Err(Config::produce_error(2)),
        };


        let dotfiles = dotfile_iter.map(|dotfile| {

                let dotfile_table = dotfile.as_table().unwrap();

                let manager_path = PathBuf::from(
                    match dotfile_table.get("manager_path") {
                        Some(path) => path.as_str().expect("Invalid character in dotfile path"),
                        None => return Err(Config::produce_error(3)),
                    }
                );

                let system_path = PathBuf::from(
                    match dotfile_table.get("system_path") {
                        Some(path) => path.as_str().expect("Invalid character in dotfile path"),
                        None => return Err(Config::produce_error(3)),
                    }
                );

                Dotfile::new(manager_path, system_path) 
        });

        Ok(dotfiles.collect())

    }


    fn get_manager_dir(config: &Table) -> PathBuf {

        let manager_dir = if config.contains_key("manager_directory") {
            PathBuf::from(config.get("manager_directory").unwrap().as_str().unwrap())
        } else {
            PathBuf::from("$HOME/.dotfiles")
        };

        manager_dir
    }


    fn produce_error(code: usize) -> Box<dyn Error> {
        let error = match code {
            1 => ConfigParseError {
                    code: 1,
                    message: String::from("No dotfiles section in config"),
                    },
            2 => ConfigParseError {
                    code: 2,
                    message: String::from("Dotfiles is not a valid config"),
                },
            3 => ConfigParseError {
                code: 3,
                message: String::from("A dotfile section in config is not valid"),
            },
            _ => ConfigParseError {
                code: 99,
                message: String::from("Error parsing config"),
            }
        };

        Box::new(error)
    }
}

struct ConfigParseError {
    code: usize,
    message: String,
}

impl Error for ConfigParseError {}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err = match self.code{
            1 => "No dotfiles section in config",
            2 => "Dotfiles section in config is not a valid array, Hint: Use [[dotfiles]]",
            3 => "A dotfile section in config is not valid",
            _ => "Error parsing config",
        };

        write!(f, "{}", err)
    }
}

impl fmt::Debug for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ConfigReadError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}
