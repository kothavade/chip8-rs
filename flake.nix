{
  description = "chip8-rs";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-stable = pkgs.rust-bin.stable.latest.default;
        nativeBuildInputs = [(rust-stable.override {extensions = ["rust-src" "rust-analyzer"];})];
        buildInputs = with pkgs; [SDL2 sccache];
      in
        with pkgs; {
          devShells.default =
            mkShell
            {
              inherit buildInputs nativeBuildInputs;
            };
        }
    );
}
