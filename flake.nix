{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    git-hooks-nix.url = "github:cachix/git-hooks.nix";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.git-hooks-nix.flakeModule
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
              cargoToml = builtins.fromTOML (builtins.readFile ./world_gen/Cargo.toml);
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
            shellHook = ''
              ${config.pre-commit.installationScript}
              echo 1>&2 "Welcome to the development shell!"
            '';
            packages = [
              rust-toolchain
              pkgs.clippy
              pkgs.hyperfine
            ];
          };
          pre-commit = {
            settings = {
              hooks = {
                deadnix.enable = true;
                statix.enable = true;
                actionlint.enable = true;
                rustfmt.enable = true;
                check-toml.enable = true;
                taplo.enable = true;
                clippy.enable = true;
              };
            };
          };

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
