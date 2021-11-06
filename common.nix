# action: 'dev' or 'build'
# 'build' will set up a minimal rust toolchain for
# building (used in `nix-build`) while 'dev' will
# setup one suitable for development (`nix-shell`).
# If on vscode, use nix env selector and point it to
# shell.nix.
# buildInputs: any extra build inputs
# maybe some packages nice to have in the shell but
# not necesary for building, for example ripgrep,
# debugger and the like.
{ action
, buildInputs ? (pkgs: [ ])
, extraToolchainComponents ? [ ]
, toolchainTargets ? (targets: [ ])
}:

{ utils ? import ./utils.nix
, pkgs ? import <nixpkgs> {
    overlays = [
      (utils.importRepo { user = "oxalica"; repo = "rust-overlay"; branch = "master"; })
    ];
  }
}:

let
  thorConfig = import ./thor-config.nix;
  thor =
    # import ../thor/default.nix
    utils.importRepo { user = "dblanovschi"; repo = "thor"; }
      { inherit pkgs; config = thorConfig; };

  buildInputs' = buildInputs pkgs;
in
with thor.rust.toolchainCommons;
thor.rust.mkRustDerivation {
  inherit action;

  pname = "lfr";
  version = "0.1.0";

  cargoLock = ./Cargo.lock;

  src = builtins.path {
    path = ./.;
    filter = path: type:
      ! builtins.any (t: t == builtins.baseNameOf path) [
        "target"
        "result"
        ".vscode"
        ".git"
        ".gitignore"
      ];
  };

  toolchain = {
    toolchain = nightly;
    inherit (toolchainTargets targets) targets defaultTarget;
  };

  inherit extraToolchainComponents;

  nativeBuildInputs = [ ];

  buildInputs = buildInputs';

  cargoAliases = {
    xtask = "run -p xtask --";
  };

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
      ctw = ctalias "cargo test --workspace";
      ctwr = ctalias "cargo test --workspace --release";

      # cargo fmt
      cf = "cargo fmt --workspace -- --emit=files";
      cx = "cargo xtask";
    };

  phases = {
    configurePhase = ''
      cargo xtask gen-syntax
    '';

    buildPhase = ''
      cargo build --release
    '';

    checkPhase = ''
      cargo test --release
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp target/x86_64-unknown-linux-musl/release/lfr $out/bin/
    '';
  };
}
