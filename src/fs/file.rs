use std::path::PathBuf;
use std::fs;
use std::error::Error;
use std::fmt;




pub struct File {
    pub path: PathBuf,
    pub filename: String,
}

impl File {
    pub fn new(path: &PathBuf) -> Result<File, FileError> {

        let parent_dir = match path.parent() {
            Some(parent) => parent,
            None => return Err(FileError::NoParentDirError),
        };

        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }

        let filename = match path.file_name() {
            Some(filename) => match filename.to_str() {
                Some(filename) => String::from(filename),
                None => return Err(FileError::FilenameInvalidUTFError),
            },
            None => return Err(FileError::NoFileNameError),
        };

        Ok(File{ path: path.to_path_buf(), filename })
    }


    pub fn copy(&self, dest_path: &PathBuf) -> Result<(), FileError> {

        fs::copy(&self.path, dest_path)?;

        Ok(())
    }

}




#[derive(Debug)]
pub enum FileError {
    CopyError(std::io::Error),
    NoFileNameError,
    NoParentDirError,
    FilenameInvalidUTFError,
}

impl Error for FileError {}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::CopyError(copy_error) => {
                write!(f, "{}", copy_error)
            },
            FileError::FilenameInvalidUTFError => {
                write!(f, "Invalild UTF in filename")
            },
            FileError::NoParentDirError => {
                write!(f, "File does not have a parent directory")
            }
            FileError::NoFileNameError => {
                write!(f, "File does not have a valid filename")
            },
        }
    }
}

impl From<std::io::Error> for FileError {
    fn from(error: std::io::Error) -> FileError {
        FileError::CopyError(error)
    }
}
