{
  description = "Nix flake for my rust experiments";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=release-24.05";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system} = {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          just
          cargo
          cargo-watch
          rust-analyzer
          rustfmt
          clippy
        ];
      };
    };
  };
}
