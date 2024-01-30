use std::fs;
use std::path::PathBuf;
use std::env;
use std::error::Error;
use clap::{Arg, Command, ArgAction, ArgMatches};

pub struct Config {
    git_location: PathBuf,
    sys_location: PathBuf,
    is_dir: bool,
}

impl Config {
    pub fn new(rel_git_location: &str, sys_location: &str) -> Result<Self, Box<dyn Error>> {

        let home_dir = PathBuf::from(env::var("HOME").expect("$HOME not set"));
        let manager_dir = home_dir.join(".dotfiles/");
        
        let git_location = manager_dir.join(rel_git_location);
        let sys_location  = PathBuf::from(sys_location);

        let is_dir = fs::metadata(&git_location)?.is_dir(); 

        Ok(Self { git_location, sys_location, is_dir })
    }

}


pub fn parse_args() -> ArgMatches {

    let matches = Command::new("dotfiles")
        .version("0.1")
        .author("Ethan Simmons")
        .about("Manages dotfiles")
        .arg(Arg::new("from-git")
            .short('g')
            .long("from-git")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    matches
}


 fn copy_config(config: &Config, to_sys: bool) -> Result<(), Box<dyn Error>> {
    if !config.is_dir {
        if to_sys {
            let _ = fs::copy(&config.git_location, &config.sys_location)?; 
            Ok(())
        } else {
            let _ = fs::copy(&config.sys_location, &config.git_location)?;
            Ok(())
        }
    } else {
        let (dir, dest) = if to_sys {
            (&config.git_location, &config.sys_location)
        } else {
            (&config.sys_location, &config.git_location)
        };

        println!("Starting Copy Dir");
        copy_directory(&dir, &dest)
    }
}


fn copy_directory(dir_path: &PathBuf, dest_path: &PathBuf) -> Result<(), Box<dyn Error>> {

    let dir = fs::read_dir(&dir_path).unwrap();

    let entries: Vec<_> = dir.map(|entry| entry.unwrap()).collect();

    let files = entries.iter().filter(|entry| entry.metadata().unwrap().is_file());
    let dirs = entries.iter().filter(|entry| entry.metadata().unwrap().is_dir());

    files.for_each(|file| {
        let file_path = dir_path.join(file.file_name());
        let dest_path = dest_path.join(file.file_name());

        let _ = fs::copy(file_path, dest_path);

        println!("Copying file");
    });

    dirs.for_each(|dir| {
        let current_dir_path = dir_path.join(dir.file_name());
        let dest_path = dest_path.join(dir.file_name());

        println!("Copying dir");
        let _ = copy_directory(&current_dir_path, &dest_path);

    });

    Ok(())

}


pub fn run(args: ArgMatches) -> Result<(), Box<dyn Error>>{

   let configs = vec![
        Config::new("desktop/sway/config", "/home/eesim/.config/sway/config")?,
        Config::new("desktop/nvim/init.lua", "/home/eesim/.config/nvim/init.lua")?,
        Config::new("desktop/nvim/lua/config", "/home/eesim/.config/nvim/lua/config")?,
        Config::new("desktop/alacritty/alacritty.toml", "/home/eesim/.config/alacritty/alacritty.toml")?,
        Config::new("desktop/rofi/config.rasi", "/home/eesim/.config/rofi/config.rasi")?,
    ];

    let copy_to_sys = args.get_flag("from-git");

    let copy_results = configs.iter().map(|config| (copy_config(&config, copy_to_sys), config));

    copy_results.for_each(|result| {
        if let Err(e) = result.0 {
            let failed_config = result.1;

            if copy_to_sys {
                println!("Faled to copy {}, with error: {}", failed_config.git_location.to_str().unwrap(), e);
            }
        }
    });


    Ok(())

}

