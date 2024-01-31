use std::path::PathBuf;
use std::error::Error;
use std::env;
use std::fs;

use crate::copy_directory;

pub struct ConfigFile {
    pub manager_path: PathBuf,
    pub system_path: PathBuf,
    is_dir: bool,
}


impl ConfigFile {
    pub fn new(rel_git_location: PathBuf, sys_location: PathBuf) -> Result<Self, Box<dyn Error>> {

        let home_dir = PathBuf::from(env::var("HOME").expect("$HOME not set"));
        let manager_dir = home_dir.join(".dotfiles/");
        
        let manager_path = manager_dir.join(rel_git_location);
        let system_path  = sys_location;

        let is_dir = fs::metadata(&manager_path)?.is_dir(); 

        Ok(Self { manager_path, system_path, is_dir })
    }

    pub fn copy_config(&self, to_sys: bool) -> Result<(), Box<dyn Error>> {

        let (curr, dest) = if to_sys {
            (&self.manager_path, &self.system_path)
        } else {
            (&self.system_path, &self.manager_path)
        };

        if !self.is_dir {
            println!("Copying file");
            fs::copy(curr, dest)?;
            Ok(())
        } else {
            println!("Starting Copy Dir");
            copy_directory(curr, dest)?;
            Ok(())
        }
    }
}
