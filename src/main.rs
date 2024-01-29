use std::fs;
use std::path::PathBuf;
use std::env;
use clap::{Arg, Command, ArgAction};

fn main() {

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

   let configs = vec![
        Config::new("desktop/sway/config", "/home/eesim/.config/sway/config"),
        Config::new("desktop/nvim/init.lua", "/home/eesim/.config/nvim/init.lua"),
        Config::new("desktop/nvim/lua/config", "/home/eesim/.config/nvim/lua/config"),
        Config::new("desktop/alacritty/alacritty.toml", "/home/eesim/.config/alacritty/alacritty.toml"),
        Config::new("desktop/rofi/config.rasi", "/home/eesim/.config/rofi/config.rasi")
    ];


    let copy_to_sys = matches.get_flag("from-git");

    configs.iter().for_each(|config| copy_config(&config, copy_to_sys));

}

struct Config {
    git_location: PathBuf,
    sys_location: PathBuf,
    is_dir: bool,
}

impl Config {
    fn new(rel_git_location: &str, sys_location: &str) -> Self {

        let home_dir = PathBuf::from(env::var("HOME").expect("$HOME not set"));
        let manager_dir = home_dir.join(".dotfiles/");
        
        let git_location = manager_dir.join(rel_git_location);
        let sys_location  = PathBuf::from(sys_location);

        let is_dir = fs::metadata(&git_location).unwrap().is_dir(); 

        Self { git_location, sys_location, is_dir }
    }
}


fn copy_config(config: &Config, to_sys: bool) {
    if !config.is_dir {
        if to_sys {
            let _ = fs::copy(&config.git_location, &config.sys_location); 
        } else {
            let _ = fs::copy(&config.sys_location, &config.git_location);
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

fn copy_directory(dir_path: &PathBuf, dest_path: &PathBuf) {

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
        copy_directory(&current_dir_path, &dest_path);

    });

}
