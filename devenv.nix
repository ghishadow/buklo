{pkgs, ...}: {
  packages = with pkgs; [git openssl cmake cargo-audit rustup cargo-cache cargo-dist cargo-insta];

  difftastic.enable = true;

  languages.nix.enable = true;
  languages.rust = {
    enable = true;
  };

  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };
}
