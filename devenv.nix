{ pkgs, ... }:

{

  packages = with pkgs; [ git openssl cmake cargo-audit rustup cargo-cache cargo-dist cargo-insta ];

  languages.nix.enable = true;
  languages.rust.enable = true;

}
