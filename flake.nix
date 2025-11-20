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
                # rust stuff
                pkgs-unstable.tracy-glfw
                pkgs-unstable.wgsl-analyzer
                rustup

                # utils
                just
                ripgrep # for justfile
                gnuplot # for benchmarks
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
                rustup show active-toolchain
              '';

            }
            // (lib.optionalAttrs stdenv.isLinux {
              # untested but probably not needed now that using rustup instead of cargo from nixpkgs
              # RUST_SRC_PATH = "${pkgs-unstable.rust.packages.stable.rustPlatform.rustLibSrc}";

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
        # iterate each system and evaluate
        eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
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
