{pkgs, ...}: {
  stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.llvmPackages_18.stdenv;

  env.CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG = true;
  env.LC_ALL = "en_US.UTF-8";
  packages = with pkgs; [
    git
    openssl
    cmake
    cargo-audit
    ccache
    sccache
    just
    cargo-cache
    cargo-dist
    cargo-insta
    cargo-audit
    cargo-auditable
    rust-analyzer
    wasmtime
    nil
    zoxide
  ];

  difftastic.enable = true;

  languages = {
    nix.enable = true;
    rust = {
      enable = true;
      channel = "nightly";
      # targets = "x86_64-unknown-linux-gnu";
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
        "rust-std"
      ];
    };
  };

  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };
}
