{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "hyprland-workspaces-tui";
  version = "1.0.2";
  src = pkgs.lib.cleanSource ./.;
  cargoHash = "sha256-RatW+9CPOmaG9u3/XCNnqyThBafZa0qRm02OvATq45I=";

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
