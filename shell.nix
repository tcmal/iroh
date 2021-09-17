{ pkgs ? import <nixpkgs> {}, lib ? pkgs.lib }:

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    cmake
    pkg-config
    freetype
    expat
    libxkbcommon

    vulkan-tools
    vulkan-loader

    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
  ];

  VK_ICD_FILENAMES = "/run/opengl-driver/share/vulkan/icd.d/nvidia_icd.json";
  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
}
