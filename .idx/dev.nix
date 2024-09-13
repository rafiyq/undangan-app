# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env

{ pkgs, ... }: 
let 
  # Rust toolchains for Nix. see: https://github.com/nix-community/fenix
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
in {

  # Which nixpkgs channel to use.
  channel = "stable-24.05"; # or "unstable"

  # Use https://search.nixos.org/packages to find packages
  packages = [
    (with fenix; combine [
      minimal.cargo
      # clippy
      # rust-src
      minimal.rustc
      # rustfmt
      targets.wasm32-unknown-unknown.latest.rust-std
    ])
    pkgs.nodejs
    pkgs.stdenv.cc
  ];

  # Sets environment variables in the workspace
  env = {
    RUST_SRC_PATH = "${fenix.stable.rust-src}/lib/rustlib/src/rust/library";
    PATH = ["$HOME/.cargo/bin"];
  };

  # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
  idx.extensions = [
    "rust-lang.rust-analyzer"
    "tamasfe.even-better-toml"
    "fill-labs.dependi"
    "vadimcn.vscode-lldb"
  ];

  # Commands to execute when the workspace is created and opened for the first time.
  idx.workspace.onCreate = {
    npm-update = "npm install -g npm@latest";
    wrangler-install = "npm install -g wrangler@latest";
    worker-build-install = "cargo install worker-build";
  };

  # Enable previews and customize configuration
  idx.previews = {
    enable = true;
    previews = {
      web = {
        command = ["wrangler dev"];
        manager = "web";
      };
    };
  };
}
