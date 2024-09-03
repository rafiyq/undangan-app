# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env

{ pkgs, ... }: {

  # Which nixpkgs channel to use.
  channel = "unstable"; # or "stable-23.11"

  # Use https://search.nixos.org/packages to find packages
  packages = with pkgs; [
    cargo
    rustc
    rustfmt
    stdenv.cc
    wasm-pack
    worker-build
    wrangler
  ];

  # Sets environment variables in the workspace
  env = {
    RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  };

  # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
  idx.extensions = [
    "rust-lang.rust-analyzer"
    "tamasfe.even-better-toml"
    "serayuzgur.crates"
    "vadimcn.vscode-lldb"
  ];

  idx.workspace = {
    onCreate = {
      wasm-install = "rustup target add wasm32-unknown-unknown";
    };
  };

  # Enable previews and customize configuration
  idx.previews = {
    enable = true;
    previews = {
      web = {
        command = [
          "wrangler"
          "dev"
        ];
        manager = "web";
      };
    };
  };
}