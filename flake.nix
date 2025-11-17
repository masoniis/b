{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-25.05";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    systems.url = "github:nix-systems/default";
    utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      nixpkgs-unstable,
      utils,
      treefmt-nix,
      systems,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        pkgs-unstable = nixpkgs-unstable.legacyPackages.${system};
      in
      {
        devShell =
          with pkgs;
          mkShell (
            {
              buildInputs = [
                # rust
                pkgs-unstable.cargo
                pkgs-unstable.tracy-glfw
                pkgs-unstable.wgsl-analyzer

                # utils
                just
                ripgrep
                gnuplot
              ]
              ++ (lib.optionals stdenv.isLinux [
                libGL
                libxkbcommon
                wayland
                pkg-config
                mesa
              ]);

              shellHook = ''
                export PATH="$HOME/.cargo/bin:$PATH"
              '';

            }
            // (lib.optionalAttrs stdenv.isLinux {
              RUST_SRC_PATH = "${pkgs-unstable.rust.packages.stable.rustPlatform.rustLibSrc}";

              LD_LIBRARY_PATH = lib.makeLibraryPath [
                libGL
                libxkbcommon
                wayland
              ];
            })
          );
      }
    )
    // (
      let
        # Small tool to iterate over each systems
        eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});

        # Eval the treefmt modules from ./treefmt.nix
        treefmtEval = eachSystem (pkgs: treefmt-nix.lib.evalModule pkgs ./treefmt.nix);
      in
      {
        # for `nix fmt`
        formatter = eachSystem (pkgs: treefmtEval.${pkgs.system}.config.build.wrapper);
        # for `nix flake check`
        checks = eachSystem (pkgs: {
          formatting = treefmtEval.${pkgs.system}.config.build.check self;
        });
      }
    );
}
