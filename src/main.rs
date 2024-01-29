use std::fs;
use std::path::PathBuf;
use std::env;

fn main() {
/*   let configs = vec![
        Config::new("laptop/sway/config", "/home/eesim/.config/sway/config"),
        Config::new("laptop/nvim/init.lua", "/home/eesim/.config/nvim/init.lua"),
        Config::new("laptop/nvim/lua/config", "/home/eesim/.config/nvim/lua/config"),
        Config::new("laptop/alacritty/alacritty.toml", "/home/eesim/.config/alacritty/alacritty.toml"),
        Config::new("laptop/rofi/config.rasi", "/home/eesim/.config/rofi/config.rasi")
    ];
*/

    let configs = vec!(Config::new("laptop/rofi/config.rasi", "/home/eesim/.config/rofi/config.rasi"));

    let _dirs = configs.iter().filter(|config| config.is_dir);
    let files  = configs.iter().filter(|config| !config.is_dir);

    files.for_each(|config| copy_file(&config, true));

}

struct Config {
    git_location: PathBuf,
    sys_location: PathBuf,
    is_dir: bool,
}

impl Config {
    fn new(rel_git_location: &str, sys_location: &str) -> Self {

        let home_dir = PathBuf::from(env::var("HOME").expect("$HOME not set"));
        let manager_dir = home_dir.join(".dotfiles/");
        
        let git_location = manager_dir.join(rel_git_location);
        let sys_location  = PathBuf::from(sys_location);

        let is_dir = fs::metadata(&git_location).unwrap().is_dir(); 

        Self { git_location, sys_location, is_dir }
    }
}

fn copy_file(config: &Config, to_sys: bool) {
    if to_sys {
        let _ = fs::copy(&config.git_location, &config.sys_location); 
    } else {
        let _ = fs::copy(&config.sys_location, &config.git_location);
    }
}
