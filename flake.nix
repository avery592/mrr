{
  inputs = {
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };
  outputs = {
    devshell,
    fenix,
    flake-utils,
    nixpkgs,
    ...
  }: (flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [
        devshell.overlays.default
        fenix.overlays.default
      ];
    };
  in {
    formatter = pkgs.alejandra;
    devShells.default = pkgs.devshell.mkShell {
      commands = [
        {package = pkgs.cargo-edit;}
        {package = pkgs.cargo-flamegraph;}
        {package = pkgs.clangStdenv;}
        {package = pkgs.fenix.stable.toolchain; }
        {package = pkgs.hyperfine;}
        {package = pkgs.nil;}
        {package = pkgs.samply;}
      ];
    };
  }));
}
