let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
      cargo
      git     
  ];
 shellHook = ''
    alias gs="git status"
    alias gc="git add -A; git commit -m"
  '';
}
