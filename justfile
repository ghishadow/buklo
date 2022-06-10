

bt := '0'

export RUST_BACKTRACE := bt

log := "warn"

export JUST_LOG := log

# create a nix shell with all tools needed for development
shell:
   nix develop .  --command "zsh"

run:
  cargo run

build:
  cargo build

release:
  cargo build --release

build-linux:
  cargo +nightly build -Z build-std=std,panic_abort  --target x86_64-unknown-linux-gnu --release

fmt:
  cargo fmt --all


spellcheck:
  cargo spellcheck check

# add git log messages to changelog
changes:
  git log --pretty=format:%s >> CHANGELOG.md


# install buklo from crates.io
install:
  cargo install -f buklo

# install development dependencies
install-dev-deps:
  rustup install nightly
  rustup update nightly
  cargo +nightly install cargo-fuzz
  cargo install cargo-check
  cargo install cargo-limit
  cargo install cargo-watch


# count non-empty lines of code
sloc:
  @cat src/*.rs | sed '/^\s*$/d' | wc -l 

fuzz:
  cargo +nightly fuzz run fuzz-compiler

# publish current GitHub master branch
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone git@github.com:ghishadow/buklo.git tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  cd tmp/release
  #git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release
