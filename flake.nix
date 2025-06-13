{
  description = "Neovim lua color scheme generator written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-25.05";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    crane,
    flake-utils,
    ...
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

      bin = craneLib.buildPackage (commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        });
    in
      with pkgs; {
        packages.default = bin;

        devShells.default = mkShell {
          inputsFrom = [bin];
          buildInputs = [rust-analyzer];
        };
      })
    // {
      inherit nixpkgs;
    };
}
