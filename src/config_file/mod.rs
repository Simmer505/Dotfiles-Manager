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
        let manager_dir = home_dir.join(PathBuf::from(".dotfiles/"));
        
        let manager_path = manager_dir.join(rel_git_location);
        let system_path  = sys_location;

        let manager_path_data = fs::metadata(&manager_path);
        let sys_path_data = fs::metadata(&system_path);

        let is_dir = match (manager_path_data, sys_path_data) {
            (Ok(manager_data), Ok(sys_data)) => manager_data.is_dir() && sys_data.is_dir(),
            (Ok(manager_data), Err(_)) => manager_data.is_dir(),
            (Err(_), Ok(sys_data)) => sys_data.is_dir(),
            (Err(e1), Err(e2)) => panic!("Neither {} nor {} exists or is readable: {}, {}", manager_path.to_str().unwrap(), system_path.to_str().unwrap(), e1, e2),
        };

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
