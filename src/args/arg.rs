use std::path::PathBuf;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use clap::{Arg, ArgAction, ArgMatches, ArgGroup, Command};



pub struct Args {
   pub copy_to_sys: bool, 
   pub dry_run: bool,
   pub override_manager_dir: Option<PathBuf>,
   pub override_device: Option<String>,
}

impl Args { 
    pub fn parse_args() -> Self {

        let cli = Args::get_cli();

        let matches = cli.get_matches();
        let values = Value::from_matches(&matches);
            
        let (flags, unprocessed) = Args::process_flags(values);
        let (args, _) = Args::process_args(unprocessed);

        let or_mgr_dir = match &args[0] {
            Some(path) => Some(PathBuf::from(path)),
            None => None,
        };
        
        Args {
            copy_to_sys: flags[0],
            dry_run: flags[1],
            override_manager_dir: or_mgr_dir,
            override_device: args[1].clone(),
        }
    }
    

    fn process_flags(values: Vec<(clap::Id, Value)> ) -> (Vec<bool>, Vec<(clap::Id, Value)>) {

        let (copy_to_sys, unmatched) = Args::get_arg(values, "from-git");
        let (dry_run, unmatched) = Args::get_arg(unmatched, "dry-run");

        let copy_to_sys = if let Value::Bool(val) = copy_to_sys.unwrap_or(Value::None) { val }
            else { false };

        let dry_run = if let Value::Bool(val) = dry_run.unwrap_or(Value::None) { val }
            else { false };


            (vec![copy_to_sys, dry_run], unmatched)
    }


    fn process_args(values: Vec<(clap::Id, Value)>) -> (Vec<Option<String>>, Vec<(clap::Id, Value)>) {

        let (override_manager_dir, unmatched) = Args::get_arg(values, "manager-dir");
        let (override_device, unmatched) = Args::get_arg(unmatched, "device");


        let or_mngr_dir = if let Value::String(val) = override_manager_dir.unwrap_or(Value::None) { Some(String::from(val)) }
            else { None };

        let or_device = if let Value::String(val) = override_device.unwrap_or(Value::None) { Some(String::from(val))}
            else { None };
         

        (vec![or_mngr_dir, or_device], unmatched)
    }
        
    

    fn get_cli() -> Command {

        let from_git = Arg::new("from-git")
            .short('f')
            .long("from-git")
            .action(ArgAction::SetTrue);

        let override_manager_dir = Arg::new("manager-dir")
            .long("manager-dir")
            .action(ArgAction::Append);

        let override_device = Arg::new("device")
            .short('d')
            .long("device")
            .action(ArgAction::Append);

        let dry_run = Arg::new("dry-run")
            .short('n')
            .long("dry")
            .action(ArgAction::SetTrue);

        let cli = Command::new("dotfiles")
            .group(ArgGroup::new("flags").multiple(true))
            .next_help_heading("FLAGS")
            .args([
                from_git,
                dry_run,
            ])
            .group(ArgGroup::new("overrides").multiple(true))
            .next_help_heading("OVERRIDES")
            .args([
                override_manager_dir,
                override_device,
            ]);

        cli
    }

    fn get_arg(args: Vec<(clap::Id, Value)>, id: &str) -> (Box<Option<Value>>, Vec<(clap::Id, Value)>) {

        let (matches, non_matches): (Vec<_>, Vec<_>) = args.into_iter().partition(|value| value.0 == id); 

        let arg_match = match matches.len() {
            0 => Box::new(None),
            1 => Box::new(Some(matches[0].1.clone())),
            _ => unreachable!(),
        };

        (arg_match, non_matches)
    } 


}




#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Value {
    Bool(bool),
    String(String),
    None,
}


impl Value {
    fn from_matches(matches: &ArgMatches) -> Vec<(clap::Id, Self)> {

        let mut values = BTreeMap::new();
        
        let _ = matches.ids().into_iter().for_each(|id| {
            let source = matches
                .value_source(id.as_str())
                .expect("id came from matches");

            if matches.try_get_many::<clap::Id>(id.as_str()).is_ok() { return () }
            if source != clap::parser::ValueSource::CommandLine { return () }
            if Self::extract::<String>(matches, id, &mut values)  { return () }
            if Self::extract::<bool>(matches, id, &mut values) { return () }
        });

        values.into_values().collect()

    }


    fn extract<T: Clone + Into<Value> + Send + Sync + 'static>(
        matches: &ArgMatches,
        id: &clap::Id,
        output: &mut BTreeMap<usize, (clap::Id, Self)>,
        ) -> bool {

        match matches.try_get_many::<T>(id.as_str()) {
            Ok(Some(values)) => {
                values.zip(
                        matches
                            .indices_of(id.as_str())
                            .expect("id came from matches")
                    )
                    .for_each(|(value, index)| {
                        output.insert(index, (id.clone(), value.clone().into()));
                    });

                true
            },
            Err(clap::parser::MatchesError::Downcast { .. }) => false,
            Ok(None) => {
                unreachable!("ids only reports what is present")
            },
            Err(_) => {
                unreachable!("id came from matches")
            },
        }
    }
}

impl From<String> for Value {
    fn from(other: String) -> Value {
        Value::String(other)
    }
}

impl From<bool> for Value {
    fn from(other: bool) -> Value {
        Value::Bool(other)
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::None
    }
}



#[derive(Debug)]
pub enum ArgError {
    ArgParseError
}

impl Error for ArgError {}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgError::ArgParseError => {
                write!(f, "Error parsing arguments")
            }
        }
    }
}
