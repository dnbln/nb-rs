{ pkgs ? import <nixpkgs> { overlays = (import ./nix/overlays.nix); } }:

pkgs.mkShell {
  nativeBuildInputs = [
    (pkgs.rust-bin.stable.latest.default.override {
      extensions = ["rust-src"];
    })
  ];

  buildInputs = [
    pkgs.nixpkgs-fmt
    pkgs.niv

    pkgs.clang_12
    pkgs.lld_12
    pkgs.glibc

    # dependencies
    pkgs.openssl
    pkgs.pkg-config
  ];

  shellHook = ''
    export CC=clang
  '';
}
