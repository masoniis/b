{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    systems.url = "github:nix-systems/default";
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
      treefmt-nix,
      systems,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell =
          with pkgs;
          mkShell (
            {
              buildInputs = [
                # Rust tools
                cargo
                rustc

                just
                wgsl-analyzer
                ripgrep
                wasm-pack
              ]
              ++ (lib.optionals stdenv.isLinux [
                libGL
                libxkbcommon
                wayland
                pkg-config
                mesa
              ]);

            }
            // (lib.optionalAttrs stdenv.isLinux {
              RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

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
