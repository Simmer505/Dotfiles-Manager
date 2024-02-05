use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::fmt;
use std::error::Error;

use crate::fs::file::{self, File};




pub struct Directory {
    files: Vec<File>,
    directories: Vec<Directory>,
    pub path: PathBuf,
    pub errors: Vec<DirError>,
}

impl Directory {
    pub fn new(path: &PathBuf) -> Result<Directory, DirError> {

        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        let entries = fs::read_dir(path)?;
        let (valid_entries, io_errors): (Vec<_>, Vec<_>) = entries.partition(|entry| entry.is_ok());
        let valid_entries: Vec<DirEntry> = valid_entries.into_iter().map(|entry| entry.unwrap()).collect();


        let dirs = Directory::get_dirs(&valid_entries);
        let (valid_dirs, dir_errors): (Vec<_>, Vec<_>) = dirs.into_iter().partition(|dir| dir.is_ok());
        let directories: Vec<Directory> = valid_dirs.into_iter().map(|dir| dir.unwrap()).collect();


        let files = Directory::get_files(&valid_entries);
        let (valid_files, file_errors): (Vec<_>, Vec<_>) = files.into_iter().partition(|file| file.is_ok());
        let files: Vec<File> = valid_files.into_iter().map(|file| file.unwrap()).collect();


        let dir_errors = dir_errors.into_iter().map(|err| DirError::from(err.err().unwrap()));
        let io_errors = io_errors.into_iter().map(|err| DirError::from(err.err().unwrap()));
        let file_errors = file_errors.into_iter().map(|err| DirError::from(err.err().unwrap()));

        let errors: Vec<DirError> = io_errors.chain(dir_errors).chain(file_errors).collect();

        Ok(Directory{ files, directories, path: path.to_path_buf(), errors })
    }


    fn get_files(entries: &Vec<DirEntry>) -> Vec<Result<File, DirError>> {

        let files: Vec<_> = entries.into_iter().filter_map(|entry| match entry.metadata() {
            Ok(data) if data.is_file() => {
                match File::new(&entry.path()) {
                    Ok(file) => Some(Ok(file)),
                    Err(e) => Some(Err(DirError::from(e))),
                }
            },
            Ok(_) => None,
            Err(e) => Some(Err(DirError::from(e))),
        }).collect();

        files
    }


    fn get_dirs(entries: &Vec<DirEntry>) -> Vec<Result<Directory, DirError>> {

        let directories: Vec<_> = entries.into_iter().filter_map(|entry| match entry.metadata() {
            Ok(data) if data.is_dir() => {
                match Directory::new(&entry.path()) {
                    Ok(dir) => Some(Ok(dir)),
                    Err(e) => Some(Err(DirError::from(e))),
                }
            },
            Ok(_) => None,
            Err(e) => Some(Err(DirError::from(e))),
        }).collect();

        directories
    }


    pub fn copy(&self, dest_path: &PathBuf) -> Result<Vec<DirError>, DirError> {

        let file_copy_results: Vec<_> = self.files
            .iter()
            .map(|file| {
                file.copy( &dest_path.join( PathBuf::from(&file.filename) ) )
        }).collect();


        let dir_copy_results = {
            let dirs = self.directories.iter();

            let results = dirs.map(|dir| {
                let dir_name = match dir.path.file_name() {
                    Some(filename) => filename,
                    None => return Err(DirError::NoDirectoryNameError),
                };

                let new_dest_path = dest_path.join(PathBuf::from(dir_name));

                if !new_dest_path.exists() {
                    fs::create_dir(&new_dest_path)?;
                }

                dir.copy(&new_dest_path)
            }).collect::<Vec<_>>();

            results
        };

        let mut copy_errors = Vec::new();

        file_copy_results.into_iter().for_each(|result| if result.is_err() {
            copy_errors.push(DirError::from(result.err().unwrap()))
        });

        dir_copy_results.into_iter().for_each(|result| match result {
            Err(e) => {
                copy_errors.push(DirError::from(e));
            },
            Ok(copy_results) => {
                copy_results.into_iter().for_each(|error| copy_errors.push(error));
            }
        });


        Ok(copy_errors)

    }
}




#[derive(Debug)]
pub enum DirError {
    DirCopyMetadataError(std::env::VarError),
    DirIOError(std::io::Error),
    DirFileCopyError(file::FileError),
    NoDirectoryNameError,
}

impl Error for DirError {}

impl fmt::Display for DirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirError::DirCopyMetadataError(var_error) => {
                write!(f, "{}", var_error)
            },
            DirError::DirIOError(io_error) => {
                write!(f, "{}", io_error)
            },
            DirError::DirFileCopyError(copy_error) => {
                write!(f, "{}", copy_error)
            },
            DirError::NoDirectoryNameError => {
                write!(f, "Directory does not have a valid name")
            }
        }
    }
}

impl From<std::env::VarError> for DirError {
    fn from(error: std::env::VarError) -> DirError {
       DirError::DirCopyMetadataError(error) 
    }
}

impl From<std::io::Error> for DirError {
    fn from(error: std::io::Error) -> DirError {
        DirError::DirIOError(error)
    }
}

impl From<file::FileError> for DirError {
    fn from(error: file::FileError) -> DirError {
        DirError::DirFileCopyError(error)
    }
}
