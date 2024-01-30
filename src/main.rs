use dotfiles_manager;


fn main() {
    let args = dotfiles_manager::parse_args();

    let _ = dotfiles_manager::run(args);


}
