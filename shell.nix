{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ gcc rustfmt rust-analyzer wasm-pack ];
}
