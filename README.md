# dotfile-manager

Manage and sync your dotfiles across multiple machines using Git

## Installation

Install only CLI

```bash
git clone https://github.com/Adamekka/dotfile-manager.git
cd dotfile-manager
make install
```

Install CLI and GUI

```bash
git clone https://github.com/Adamekka/dotfile-manager.git
cd dotfile-manager
make install-gui
```

## Usage

In your `~/.config/` directory, `dotfile-manager` directory is going to be created, inside it's going to be created `templates` directory, where your templates are going to be stored.

### CLI

| command  | arguments  | subcommands   | description                                      |
| -------- | ---------- | ------------- | ------------------------------------------------ |
| new      | -n, -g, -p | _none_        | Create new template                              |
| list     | _none_     | _none_        | List all templates                               |
| import   | _none_     | file          | Import template(s) from toml file                |
| export   | _none_     | file          | Export all template(s) to toml file              |
| remove   | -n, -g, -p | template name | Remove template from dman, not from filesystem   |
| pull     | -n, -g, -p | template name | Clone template and pull changes from remote      |
| pull-all | _none_     | _none_        | Clone all templates and pull changes from remote |

#### Arguments

> Those are used only at `new`, `remove` and `pull` commands

| short argument | long argument          | example                               | description |
| -------------- | ---------------------- | ------------------------------------- | ----------- |
| -n \<name>     | --name \<name>         | -n nvim                               | Name        |
| -p \<path>     | --path \<path>         | -p ~/.config/nvim                     | Path        |
| -g \<git-path> | --git-path \<git-path> | -g <https://github.com/Adamekka/nvim> | Git repo    |

### GUI

> todo
