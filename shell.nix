let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
      /* cargo git vim */
  ];
 shellHook = /* bash */''
    export PROJECT_ROOT=${toString ./.}
    alias gs="git status"
    alias gc="git add -A; git commit -m"
    alias todo="vim $PROJECT_ROOT/TODO.md"
    alias readme="vim $PROJECT_ROOT/README.md"
    alias src="vim $PROJECT_ROOT/src"
  '';
}
