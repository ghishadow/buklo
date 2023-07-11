{ pkgs, ... }:

{

  packages = with pkgs; [ git ];



  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;

}
