use std::path::PathBuf;
use std::error::Error;
use std::env;
use std::fs;
use std::fmt;

use crate::fs::dir;
use crate::fs::file;





pub enum Dotfile {
    File(file::File),
    Dir(dir::Directory)
}


pub struct ManagedDotfile {
    pub manager_dotfile: Dotfile,
    pub system_dotfile: Dotfile,
}

impl ManagedDotfile {
    pub fn new(rel_git_location: PathBuf, sys_location: PathBuf) -> Result<Self, DotfileError> {

        let home_dir = PathBuf::from(env::var("HOME")?);
        let manager_dir = home_dir.join(PathBuf::from(".dotfiles/"));
        
        let manager_path = manager_dir.join(rel_git_location);
        let system_path  = sys_location;

        let manager_path_data = fs::metadata(&manager_path);
        let sys_path_data = fs::metadata(&system_path);

        let is_dir = match (manager_path_data, sys_path_data) {
            (Ok(manager_data), Ok(sys_data)) => manager_data.is_dir() && sys_data.is_dir(),

            (Ok(manager_data), Err(_)) => {
                if manager_data.is_dir() {
                    let _ = fs::create_dir_all(&system_path);
                    true
                } else {
                    let _ = fs::create_dir_all(&system_path.parent().unwrap());
                    false
                }
            },

            (Err(_), Ok(sys_data)) =>  {
                if sys_data.is_dir() {
                    let _ = fs::create_dir_all(&manager_path);
                    true
                } else {
                    let _ = fs::create_dir_all(&manager_path.parent().unwrap());
                    false
                }
            },

            (Err(e1), Err(e2)) => return Err(DotfileError::FilesDontExistError((e1, e2)))
        };

        let manager_dotfile = if is_dir {
            Dotfile::Dir(dir::Directory::new(&manager_path)?)
        } else {
            Dotfile::File(file::File::new(&manager_path)?)
        };

        let system_dotfile = if is_dir {
            Dotfile::Dir(dir::Directory::new(&system_path)?)
        } else {
            Dotfile::File(file::File::new(&system_path)?)
        };

        Ok(Self { manager_dotfile, system_dotfile })
    }


    pub fn copy_dotfile(&self, to_sys: bool) -> Result<(), DotfileError> {

        let (current, destination) = if to_sys {
            (&self.manager_dotfile, &self.system_dotfile)
        } else {
            (&self.system_dotfile, &self.manager_dotfile)
        };


        if let (Dotfile::File(current_file), Dotfile::File(dest_file)) = (current, destination) {
            current_file.copy(&dest_file.path)?;
        };


        if let (Dotfile::Dir(current_dir), Dotfile::Dir(dest_dir)) = (current, destination) {
            let results = current_dir.copy(&dest_dir.path)?;

            results.into_iter().for_each(|result| {
                println!("Error copying directory: {}", result)
            })
        };

        Ok(())
    }
}





#[derive(Debug)]
pub enum DotfileError {
    DotfileIOError(std::io::Error),
    DotfileEnvError(std::env::VarError),
    FilesDontExistError((std::io::Error, std::io::Error)),
    FileCopyError(file::FileError),
    DirectoryCopyError(dir::DirError),
}

impl Error for DotfileError {}

impl fmt::Display for DotfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DotfileError::DotfileIOError(io_error) => {
                write!(f, "{}", io_error)
            },
            DotfileError::DotfileEnvError(env_error) => {
                write!(f, "{}", env_error)
            }
            DotfileError::FilesDontExistError((io_error_1, io_error_2)) => {
                write!(f, "Neither file exists: {}, {}", io_error_1, io_error_2)
            }
            DotfileError::FileCopyError(copy_error) => {
                write!(f, "{}", copy_error)
            }
            DotfileError::DirectoryCopyError(copy_error) => {
                write!(f, "{}", copy_error)
            }
        }
    }
}

impl From<std::io::Error> for DotfileError {
    fn from(error: std::io::Error) -> DotfileError {
        DotfileError::DotfileIOError(error)
    }
}

impl From<std::env::VarError> for DotfileError {
    fn from(error: std::env::VarError) -> DotfileError {
        DotfileError::DotfileEnvError(error)
    }
}

impl From<(std::io::Error, std::io::Error)> for DotfileError {
    fn from(error: (std::io::Error, std::io::Error)) -> DotfileError {
        DotfileError::FilesDontExistError(error)
    }
}

impl From<file::FileError> for DotfileError {
    fn from(error: file::FileError) -> DotfileError {
        DotfileError::FileCopyError(error)
    }
}

impl From<dir::DirError> for DotfileError {
    fn from(error: dir::DirError) -> DotfileError {
        DotfileError::DirectoryCopyError(error)
    }
}
