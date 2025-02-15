{
  nixpkgs,
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  languages.rust = {
    channel = "nightly";
    mold.enable = true;
    components = ["rustc" "rust-src" "cargo" "clippy" "rustfmt" "rust-analyzer"];
    enable = true;
  };

  packages = [pkgs.gcc pkgs.cmake];

  env.LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath [
    # pkgs.xorg.libX11
    # pkgs.xorg.libXcursor
    # pkgs.xorg.libxcb
    # pkgs.xorg.libXi
    pkgs.gcc
    pkgs.libxkbcommon
    pkgs.libGL
    pkgs.libxkbcommon
    pkgs.wayland
  ]}";
}
