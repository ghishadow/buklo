{pkgs, ...}: {
  env.CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true;
  env.LC_ALL="en_US.UTF-8";
  packages = with pkgs; [
    git
    openssl
    cmake
    cargo-audit
    rustup
    cargo-cache
    cargo-dist
    cargo-insta
    cargo-audit
    cargo-auditable
    clang_17
    llvmPackages_17.stdenv
    lld_17
    rust-analyzer
    nil
    zoxide
    ];

  difftastic.enable = true;

  languages.nix.enable = true;

  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };
}
