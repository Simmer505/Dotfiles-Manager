use std::path::PathBuf;
use std::error::Error;
use std::env;
use std::fs;
use std::fmt;
use std::io;

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

        let manager_is_dir = ManagedDotfile::check_is_dir(&manager_path)?;
        let sys_is_dir = ManagedDotfile::check_is_dir(&system_path)?;


        let is_dir = match (manager_is_dir, sys_is_dir) {
            (Some(sys_is_dir), Some(manager_is_dir)) if sys_is_dir && manager_is_dir => true,
            (Some(sys_is_dirl), None) if sys_is_dirl => true,
            (None, Some(manager_is_dir)) if manager_is_dir => true,
            _ => false,
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


    fn check_is_dir(path: &PathBuf) -> Result<Option<bool>, DotfileError> {

        let path_is_dir = match fs::metadata(&path) {
            Ok(data) if data.is_dir() => Some(true),
            Ok(_) => Some(false),
            Err(e) if e.kind() == io::ErrorKind::NotFound => None,
            Err(e) => return Err(DotfileError::from(e)),
        };

        Ok(path_is_dir)

    }


    pub fn copy_dotfile(&self, to_sys: bool) -> Result<Vec<dir::DirError>, DotfileError> {

        let (current, destination) = if to_sys {
            (&self.manager_dotfile, &self.system_dotfile)
        } else {
            (&self.system_dotfile, &self.manager_dotfile)
        };


        let copy_results = if let (Dotfile::File(current_file), Dotfile::File(dest_file)) = (current, destination) {
            current_file.copy(&dest_file.path)?;
            Vec::new()
        } else if let (Dotfile::Dir(current_dir), Dotfile::Dir(dest_dir)) = (current, destination) {
            current_dir.copy(&dest_dir.path)?
        } else {
            return Err(DotfileError::DotfileCopyError)
        };

        Ok(copy_results)
    }
}





#[derive(Debug)]
pub enum DotfileError {
    DotfileIOError(std::io::Error),
    DotfileEnvError(std::env::VarError),
    FileCopyError(file::FileError),
    DirectoryCopyError(dir::DirError),
    FilesDontExistError,
    DotfileCopyError,
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
            },
            DotfileError::FileCopyError(copy_error) => {
                write!(f, "{}", copy_error)
            },
            DotfileError::DirectoryCopyError(copy_error) => {
                write!(f, "{}", copy_error)
            },
            DotfileError::FilesDontExistError => {
                write!(f, "Neither file exists")
            },
            DotfileError::DotfileCopyError => {
                write!(f, "Failed to copy dotfile")
            },
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
