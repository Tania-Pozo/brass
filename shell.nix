let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
      cargo
      git 
  ];
 shellHook = /* bash */''
    export PROJECT_ROOT=${toString ./.}
    alias gs="git status"
    alias gc="git add -A; git commit -m"
    alias todo="vim TODO.md"
    alias readme="vim README.md"
    alias src="vim src"
  '';
}
