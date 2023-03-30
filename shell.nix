with import <nixpkgs> {};
  mkShell {
    name = "rust";
    packages = [
      rust-analyzer
      rustup
    ];
  }
