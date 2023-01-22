{
  description = "buklo";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-linux" "aarch64-darwin"] (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            cargo-outdated
            cargo-edit
            cargo-audit
            cargo-deny
            cargo-nextest
            jo
            rust-analyzer
            rustfmt
            # Rust
            # Shells
            nushell
            # to test github actions
            act
            # Tools
            helix
            nixfmt
            cargo-cross
            # Dependencies
            cacert
            openssl
            git
            git-lfs
            mosh
            zlib
            pkg-config
          ];
        };
      }
    );
}
