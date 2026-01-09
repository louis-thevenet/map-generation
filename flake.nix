{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    git-hooks-nix.url = "github:cachix/git-hooks.nix";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.git-hooks-nix.flakeModule
      ];
      perSystem =
        {
          config,
          pkgs,
          lib,
          ...
        }:
        let
          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = with pkgs; [
              rustc
              rustfmt
              cargo
              cargo-watch
              rust-analyzer
              rustPlatform.rustcSrc
              cargo-dist
              cargo-tarpaulin
              cargo-insta
              cargo-machete
              cargo-edit
              cargo-flamegraph
              clippy
              hyperfine
              flamelens
            ];
          };
          bevy_deps = with pkgs; [
            pkg-config
            # for Linux
            # Audio (Linux only)
            alsa-lib
            # Cross Platform 3D Graphics API
            vulkan-loader
            # For debugging around vulkan
            vulkan-tools
            # Other dependencies
            libudev-zero
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland
          ];

        in
        {
          packages.world-gen =
            let
              cargoToml = builtins.fromTOML (builtins.readFile ./world_gen/Cargo.toml);
            in
            pkgs.rustPlatform.buildRustPackage rec {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              cargoBuildFlags = "-p " + name;
            };
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

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            RUST_BACKTRACE = "full";
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
            shellHook = ''
              ${config.pre-commit.installationScript}
              echo 1>&2 "Welcome to the development shell!"
            '';
            LD_LIBRARY_PATH = lib.makeLibraryPath (
              with pkgs;
              [
                vulkan-loader
                xorg.libX11
                xorg.libXi
                xorg.libXcursor
                libxkbcommon
              ]
            );
            packages = [
              rust-toolchain
            ]
            ++ bevy_deps;
          };
          pre-commit = {
            settings = {
              settings.rust.check.cargoDeps = pkgs.rustPlatform.importCargoLock {
                lockFile = ./Cargo.lock;
              };
              hooks = {
                deadnix.enable = true;
                statix.enable = true;
                actionlint.enable = true;
                rustfmt = {
                  enable = true;
                  packageOverrides = {
                    inherit (pkgs) cargo;
                    inherit (pkgs) rustfmt;
                  };
                };
                check-toml.enable = true;
                taplo.enable = true;
                typos = {
                  enable = true;
                  settings = {
                    ignored-words = [ "ratatui" ];
                  };
                };
                clippy = {
                  enable = true;
                  packageOverrides = {
                    inherit (pkgs) cargo;
                    inherit (pkgs) clippy;
                  };
                };
              };
            };

          };
        };
    };
}
