{
  description = "A flake for building the Alkahest game engine";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      code = pkgs.callPackage ./. { inherit pkgs system rust-overlay; };
    in rec {
      packages = {
        app = code.app;
        default = packages.app;
      };
      devShells = {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            cmake
            gnumake
            extra-cmake-modules
            wayland
            wayland-protocols
            libxkbcommon
          ];
          shellHook = ''
            export RUST_LOG=trace
          '';
        };
      };
    }
  );
}
