{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.stable.latest.default.override {
            extensions = [
              "clippy"
              "rust-docs"
              "rust-src"
            ];
            targets = [
              "riscv32imac-unknown-none-elf"
            ];
          }
        );

        my-crate = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          doCheck = false;
        };
      in
      {
        packages.default = my-crate;

        devShells.default = craneLib.devShell {
          packages = [
            pkgs.probe-rs-tools
          ];
        };
      }
    );
}
