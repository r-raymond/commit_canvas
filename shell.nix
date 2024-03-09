{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ gcc rustfmt clippy rust-analyzer wasm-pack inotify-tools];
}
