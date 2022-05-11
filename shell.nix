{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    buildInputs = [
      # Rust
      #pkgs.cargo
      #pkgs.rustc
      pkgs.rustup
      # Shells
      pkgs.zsh

      # Tools
      pkgs.docker
      pkgs.cargo-audit
      pkgs.nixfmt
      pkgs.cargo-cross
      # Dependencies
      pkgs.cacert
      pkgs.openssl
      pkgs.git
      pkgs.zlib
      pkgs.pkg-config
    ];
    RUST_BACKTRACE = 1;
  }
