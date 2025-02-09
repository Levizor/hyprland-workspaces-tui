# hyprland-workspaces-tui
[![Packaging status](https://repology.org/badge/version-for-repo/nix_unstable/hyprland-workspaces-tui.svg)](https://search.nixos.org/packages?channel=unstable&show=<your-package-name>)

hyprland-workspaces-tui is a terminal-based user interface (TUI) wrapper for the [hyprland-workspaces](https://github.com/FieldofClay/hyprland-workspaces) CLI utility. It provides a sleek and efficient way to display your Hyprland workspaces directly in your terminal.

Designed as a lightweight alternative to common bar functionalities.


## Installation 
Available in nixpkgs-unstable.
```
  environment.systemPackages = with pkgs; [
    hyprland-workspaces-tui
  ];
```
  
The other option requires [hyprland-workspaces](https://github.com/FieldofClay/hyprland-workspaces) to be installed manually.
```
cargo install hyprland-workspaces-tui
```


## Showcase
![](images/1.png)
![](images/2.png)
![](images/3.png)


