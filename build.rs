use std::process::Command;

fn main() {
    if Command::new("hyprland-workspaces").output().is_err() {
        eprintln!("Error: 'hyprland-workspaces' is not installed or not in PATH");
        std::process::exit(1);
    }
}
