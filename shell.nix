{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ gcc rustfmt rustup rust-analyzer wasm-pack ];
}
