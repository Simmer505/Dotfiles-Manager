# Dotfiles-Manager

## A manager for my dotfiles

Currently in the early prototype stage

## Usage

### Copies to manager folder

```
dotfile
```

### Copies from manager folder

```
dotfile -f
```

## Functionality

### Currently implemented

* Copy files to and from a folder
* Parses locations from toml config

### Future

* Select which configs to copy
* Allow copying configs from other devices
* Automatically push and pull if folder is a git repo

## Config

$HOME/.config/dotfiles/config

```
# Optional
manager_dir="Path to manager dir"                # default: "$HOME/.dotfiles"

[[dotfiles]]
system_location="Config location on system"      # example: "/home/user/.config/program/config.cfg"
manager_location="Config relative to manager"    # example: "program/config.cfg" 

[[dotfiles]]
system_location="Next system config location"
manager_location="Next manager config location"

...

```
