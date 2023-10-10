{
  description = "Neovim lua color scheme generator written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        flake-utils.follows = "flake-utils";
      };
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    crane,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};

      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain;

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = craneLib.cleanCargoSource ./.;

      nativeBuildInputs = with pkgs; [rustToolchain pkg-config];

      buildInputs = with pkgs; [openssl sqlite];

      commonArgs = {
        inherit src buildInputs nativeBuildInputs;
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      bin = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });

      dockerImage = pkgs.dockerTools.buildLayeredImage {
        name = "huey";
        tag = "latest";

        contents = [bin];

        config = {
          Cmd = ["${bin}/bin/huey"];
        };
      };
    in
      with pkgs; {
        packages = {
          inherit bin dockerImage;
          default = bin;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [bin];
          buildInputs = [rust-analyzer];
        };
      });
}
