{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ gcc rustfmt rustup wasm-pack llvm ];
}
