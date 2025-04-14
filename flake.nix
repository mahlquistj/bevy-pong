# Thanks to: https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10#a-flake-with-a-dev-shell
{
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
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
          ];
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [rustToolchain];
            RUST_LOG = "debug";
            RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = libPath;
            packages = [
              bacon
              cargo-nextest
              alsa-lib
              pkg-config
              libudev-zero
              vulkan-loader
              xorg.libX11
              xorg.libXrandr
              xorg.libXcursor
              xorg.libXi
            ];
          };
        }
    );
}
