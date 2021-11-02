{ utils ? import ./utils.nix
, pkgs ? import <nixpkgs> {
    overlays = [
      (utils.importRepo { user = "oxalica"; repo = "rust-overlay"; branch = "master"; })
    ];
  }
}:

let
  thorConfig = import ./thor-config.nix;
  thor = utils.importRepo { user = "dblanovschi"; repo = "thor"; } { inherit pkgs; config = thorConfig; };
in
with thor.rust.toolchainCommons;
thor.rust.mkRustShell {
  toolchain = "nightly-musl";

  extraNativeBuildInputs = [ ];

  extraBuildInputs = [
    pkgs.nixpkgs-fmt
  ];
}