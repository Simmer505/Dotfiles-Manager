use std::fs;
use std::str;
use std::path::PathBuf;
use std::error::Error;
use toml::Table;

use crate::dotfile::Dotfile;


pub struct Config {
    pub manager_dir: PathBuf,
    pub dotfiles: Vec<Dotfile>,
}


impl Config {
    pub fn parse(config: PathBuf) -> Result<Self, Box<dyn Error>> {

        let config_file: Table = str::from_utf8(&fs::read(config)?)?.parse()?;

        let read_dotfiles = config_file.get("dotfiles").expect("No dotfiles section in config");

        let dotfile_array = read_dotfiles.as_array().expect("Invalid config file format").iter();

        let dotfiles = dotfile_array.map(|dotfile| {

                let dotfile_table = dotfile.as_table().unwrap();

                let manager_path = PathBuf::from(
                    match dotfile_table.get("manager_path") {
                        Some(path) => path.as_str().expect("Invalid character in dotfile path"),
                        None => return None,
                    }
                );

                let system_path = PathBuf::from(
                    match dotfile_table.get("system_path") {
                        Some(path) => path.as_str().expect("Invalid character in dotfile path"),
                        None => return None,
                    }
                );

                match Dotfile::new(manager_path, system_path) {
                    Ok(dotfile) => Some(dotfile),
                    Err(e) => {
                        println!("Failed to read dotfile: {}", e);
                        None
                    }
                }
        });

        let valid_dotfiles: Vec<Dotfile> = dotfiles.filter_map(|dotfile| match dotfile {
            Some(dotfile) => Some(dotfile),
            None => {
                println!("Failed to parse config");
                None
            },
        }).collect();

        let manager_dir = if config_file.contains_key("manager_directory") {
            PathBuf::from(config_file.get("manager_directory").unwrap().as_str().unwrap())
        } else {
            PathBuf::from("$HOME/.dotfiles")
        };

        Ok(Config{manager_dir, dotfiles: valid_dotfiles})
    }
}
