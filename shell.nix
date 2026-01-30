{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell rec {
  buildInputs = [
    pkgs.pkg-config
    pkgs.gtk4
    pkgs.librsvg
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
