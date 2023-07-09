{
  description = "Simple GUI VLSM calculator.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};
        libPath = with pkgs; lib.makeLibraryPath [
            vulkan-loader
            libGL 
            libxkbcommon
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
        ];

      in {
        devShell = with pkgs; mkShell {
          buildInputs = [
            cmake
            pkg-config
            fontconfig
            cargo
            rustc
            rustfmt
            rust-analyzer
            rustPackages.clippy
          ];

          LD_LIBRARY_PATH = libPath;
        };

        defaultPackage = with pkgs; naersk'.buildPackage {
          src = ./.;
          pname = "vlsm-calc";
          nativeBuildInputs = [
            makeWrapper
          ];

          buildInputs = [
            cmake
            pkg-config
            fontconfig
          ];

          postInstall = ''
            wrapProgram "$out/bin/vlsm-calc" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';

          LD_LIBRARY_PATH = libPath;
        };

        defaultApp = flake-utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };
      }
    );
}
