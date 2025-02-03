{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "hyprland-workspaces-tui";
  version = "1.0.2";
  src = pkgs.lib.cleanSource ./.;
  cargoHash = "sha256-0bADed6AUMx5UkJ3oqxthaKt94ocEn7xXsgaH0wOiNM=";

  buildInputs = [
    pkgs.makeWrapper
    pkgs.hyprland-workspaces
  ];

  postFixup = ''
    wrapProgram $out/bin/hyprland-workspaces-tui \
      --set PATH ${pkgs.hyprland-workspaces}/bin:$PATH
  '';

  meta = with pkgs.lib; {
    description = "terminal-based user interface (TUI) wrapper for the hyprland-workspaces
      CLI utility.";
    homepage = "https://github.com/Levizor/hyprland-workspaces-tui";
    license = licenses.mit;
  };
}
