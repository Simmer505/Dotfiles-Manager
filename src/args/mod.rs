use clap::{Arg, Command, ArgAction, ArgMatches};


pub fn parse_args() -> ArgMatches {

    let matches = Command::new("dotfiles")
        .version("0.1")
        .author("Ethan Simmons")
        .about("Manages dotfiles")
        .arg(Arg::new("from-git")
            .short('f')
            .long("from-git")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    matches
}
