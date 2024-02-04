use std::fs;
use std::path::PathBuf;
use std::fmt;
use std::error::Error;

use crate::dotfile::file::{self, File};




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

        let dir: Vec<_> = fs::read_dir(path)?.collect();

        // Find a better way to do this sometime
        let mut read_errors: Vec<DirError> = Vec::new();
        let mut metadata_errors: Vec<DirError> = Vec::new();
        let mut create_dir_errors: Vec<DirError> = Vec::new();
        let mut create_file_errors: Vec<DirError> = Vec::new();

        let entries = dir.into_iter().filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(e) => {
                read_errors.push(DirError::from(e));
                None
            }
        });

        
        let valid_entries: Vec<_> = entries
            .filter_map(|entry| match entry.metadata() {
                Ok(_) => Some(entry),
                Err(e) => {
                    metadata_errors.push(DirError::from(e));
                    None
                }
            })
            .collect();


        let directories: Vec<_> = valid_entries
            .iter()
            .filter_map(|entry|
                if entry.metadata().unwrap().is_dir() {
                    match Directory::new(&entry.path()) {
                        Ok(dir) => Some(dir),
                        Err(e) => {
                            create_dir_errors.push(DirError::from(e));
                            None
                        },
                    }
            } else {
                None
            })
            .collect();


        let files: Vec<File> = valid_entries
            .iter()
            .filter_map(|entry| 
                if entry.metadata().unwrap().is_file() {
                    match File::new(&entry.path()) {
                        Ok(file) => Some(file),
                        Err(e) => {
                            create_file_errors.push(DirError::from(e));
                            None
                        },
                    }
            } else {
                None
            })
            .collect();

        // Fix sometime
        let errors: Vec<DirError> = read_errors.into_iter().chain(metadata_errors.into_iter().chain(create_dir_errors.into_iter().chain(create_file_errors.into_iter()))).collect();


        Ok(Directory{ files, directories, path: path.to_path_buf(), errors })
    }


    pub fn copy(&self, dest_path: &PathBuf) -> Result<Vec<DirError>, DirError> {

        let file_copy_results: Vec<_> = self.files
            .iter()
            .map(|file| {
                file.copy( &dest_path.join( PathBuf::from(&file.filename) ) )
                })
            .collect();


        let dir_copy_results = {
            let dirs = self.directories.iter();

            let result = dirs.map(|dir| {
                let dir_name = match dir.path.file_name() {
                    Some(filename) => filename,
                    None => return Err(DirError::NoDirNameError),
                };

                let new_dest_path = dest_path.join(PathBuf::from(dir_name));

                if !new_dest_path.exists() {
                    fs::create_dir(&new_dest_path)?;
                }

                dir.copy(&new_dest_path)
            }).collect::<Vec<_>>();

            result
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
    NoDirNameError,
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
            DirError::NoDirNameError => {
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
