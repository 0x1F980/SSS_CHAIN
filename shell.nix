# shell.nix - Hermetic Dev Shell for SSS_CHAIN
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "sss-chain-build-env";
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    musl
  ];

  shellHook = ''
    export RUSTFLAGS="-C target-feature=+aes,+ssse3 -C link-arg=-s"
    echo "Hermetic SSS_CHAIN Nix Environment Loaded!"
  '';
}
