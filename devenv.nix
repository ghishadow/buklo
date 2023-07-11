{ pkgs, ... }:

{

  packages = with pkgs; [ git openssl ];

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;

}
