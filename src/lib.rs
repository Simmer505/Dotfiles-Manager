use std::fs;
use std::path::{self, PathBuf};
use std::error::Error;
use clap::ArgMatches;

pub mod config_file;
pub mod program_config;
pub mod args;

use program_config::ProgramConfig;

fn copy_directory(dir_path: &PathBuf, dest_path: &PathBuf) -> Result<(), Box<dyn Error>> {

    let dir = fs::read_dir(&dir_path)?;

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

        if !(path::Path::try_exists(&dest_path).unwrap()) {
            let _ = fs::create_dir(&dest_path);
        }


        println!("Copying dir");
        let _ = copy_directory(&current_dir_path, &dest_path);

    });

    Ok(())

}





pub fn run(args: ArgMatches, program_config: ProgramConfig) -> Result<(), Box<dyn Error>> {

    let configs = program_config.configs;

    let copy_to_sys = args.get_flag("from-git");

    let copy_results = configs.iter().map(|config| (config.copy_config(copy_to_sys), config));

    copy_results.for_each(|result| {
        if let Err(e) = result.0 {
            let failed_config = result.1;

            if copy_to_sys {
                println!("Faled to copy {}, with error: {}", failed_config.manager_path.to_str().expect("Error printing error"), e);
            }
        }
    });

    Ok(())
}

