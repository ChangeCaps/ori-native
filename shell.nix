{ pkgs ? import <nixpkgs> {
  config = {
    allowUnfree = true;
    android_sdk.accept_license = true;
  };
} }:

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    pkg-config
    gtk4
    gtk4-layer-shell
    librsvg
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
