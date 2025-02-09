{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];
      perSystem =
        {
          config,
          pkgs,
          ...
        }:
        let
          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = with pkgs; [
              rustc
              cargo
              cargo-watch
              rust-analyzer
              rustPlatform.rustcSrc
              cargo-dist
              cargo-tarpaulin
              cargo-insta
              cargo-machete
              cargo-edit
            ];
          };

        in
        rec {
          # Rust package
          packages.world_viewer =
            let
              cargoToml = builtins.fromTOML (builtins.readFile ./world_viewer/Cargo.toml);
            in
            pkgs.rustPlatform.buildRustPackage rec {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              cargoBuildFlags = "-p " + name;
            };
          packages.default = packages.world_viewer;
          packages.perlin_image =
            let
              cargoToml = builtins.fromTOML (builtins.readFile ./perlin_to_image/Cargo.toml);
            in
            pkgs.rustPlatform.buildRustPackage rec {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              cargoBuildFlags = "-p " + name;

            };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.treefmt.build.devShell
            ];
            RUST_BACKTRACE = "full";
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

            packages = [
              rust-toolchain
              pkgs.clippy
              pkgs.hyperfine
            ];
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
              toml-sort.enable = true;
            };
          };
        };
    };
}
