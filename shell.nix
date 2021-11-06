{ utils ? import ./utils.nix
, pkgs ? import <nixpkgs> {
    overlays = [
      (utils.importRepo { user = "oxalica"; repo = "rust-overlay"; branch = "master"; })
    ];
  }
}:

let
  thorConfig = import ./thor-config.nix;
  thor = utils.importRepo { user = "dblanovschi"; repo = "thor"; }
    { inherit pkgs; config = thorConfig; };
in
with thor.rust.toolchainCommons;
thor.rust.mkRustDerivation {
  action = "dev";

  name = "nb-rs-shell";

  toolchain = {
    toolchain = nightly;
    targets = [
      targets.x86_64-linux-gnu
      targets.x86_64-linux-musl
    ];
    defaultTarget = targets.x86_64-linux-musl;
  };

  buildInputs = with pkgs; [ nixpkgs-fmt ];

  cargoAliases = { };

  enableIncremental = true;

  shellAliases =
    let
      ctalias = alias: {
        inherit alias;
        # has to modify env before running cargo to run doctests
        isCargoTest = true;
      };
    in
    {
      # cargo run
      cr = "cargo run";
      crr = "cargo run --release";

      # cargo build
      cb = "cargo build";
      cbr = "cargo build --release";

      # cargo test
      ct = ctalias "cargo test";
      ctr = ctalias "cargo test --release";

      # cargo fmt
      cf = "cargo fmt --workspace -- --emit=files";
    };

  phases.build = false;
}
