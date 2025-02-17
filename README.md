# hyprland-workspaces-tui

[![Packaging status](https://repology.org/badge/vertical-allrepos/hyprland-workspaces-tui.svg)](https://repology.org/project/hyprland-workspaces-tui/versions)

hyprland-workspaces-tui is a terminal-based user interface (TUI) wrapper for the [hyprland-workspaces](https://github.com/FieldofClay/hyprland-workspaces) CLI utility. It provides a sleek and efficient way to display your Hyprland workspaces directly in your terminal.

Designed as a lightweight alternative to common status bar functionalities.

## Installation

### NixOS or Nix package manager

Available in nixpkgs-unstable.

```
  environment.systemPackages = with pkgs; [
    hyprland-workspaces-tui
  ];
```

### Arch linux

Available in AUR but requires hyprland-workspaces to be installed.

```
yay -S hyprland-workspaces hyprland-workspaces-tui
```

### Other

The other option requires [hyprland-workspaces](https://github.com/FieldofClay/hyprland-workspaces) to be installed manually.

```
cargo install hyprland-workspaces-tui
```

or you can install both using cargo

```
cargo install hyprland-workspaces hyprland-workspaces-tui
```

Don't forget to add ~/.cargo/bin to your PATH.

## Configuration

Config file is read from $XDG_CONFIG_HOME by default, you can specify config path with -c flag.
Cli options that are clones of config options will override config definitions.
Look for options in [config example](./config_example.toml).

## Showcase

![](images/1.png)
![](images/2.png)
![](images/3.png)
