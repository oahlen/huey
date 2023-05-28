{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain = let
            rust = super.rust-bin;
          in
            if builtins.pathExists ./rust-toolchain.toml
            then rust.fromRustupToolchainFile ./rust-toolchain.toml
            else if builtins.pathExists ./rust-toolchain
            then rust.fromRustupToolchainFile ./rust-toolchain
            else rust.stable.latest.default;
        })
      ];

      pkgs = import nixpkgs {inherit system overlays;};
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          bacon
          cargo-deny
          cargo-edit
          cargo-watch
          openssl
          pkg-config
          rust-analyzer
          rustToolchain
        ];

        shellHook = ''
          Using rust version:
          ${pkgs.rustToolchain}/bin/cargo --version
        '';
      };
    });
}
