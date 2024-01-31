use std::fs;
use std::str;
use std::path::PathBuf;
use std::error::Error;
use toml::Table;

use crate::config_file::ConfigFile;


pub struct ProgramConfig {
    pub manager_dir: PathBuf,
    pub configs: Vec<ConfigFile>,
}


impl ProgramConfig {
    pub fn parse(config: PathBuf) -> Result<Self, Box<dyn Error>> {

        let config_file: Table = str::from_utf8(&fs::read(config)?)?.parse()?;

        let configs = {
            let read_configs = config_file.get("configs").unwrap();

            let configs = read_configs.as_array().unwrap().iter();

            configs.map(|config| {

                    let table = config.as_table().unwrap();
                    let manager_path = PathBuf::from(table.get("manager_path").unwrap().as_str().unwrap());
                    let system_path = PathBuf::from(table.get("system_path").unwrap().as_str().unwrap());

                    println!("Manager Path: {}", table.get("manager_path").unwrap().as_str().unwrap());
                    println!("System Path: {}", table.get("system_path").unwrap().as_str().unwrap());

                    Some(ConfigFile::new(manager_path, system_path).unwrap())
            })


        };

        let valid_configs: Vec<ConfigFile> = configs.filter_map(|config| match config {
            Some(config) => Some(config),
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

        Ok(ProgramConfig{manager_dir, configs: valid_configs})
    }
}
