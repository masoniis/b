{ ... }:
{
  projectRootFile = "flake.nix";
  programs = {
    clang-format = {
      enable = true;
      includes = [ "*.wgsl" ];
    };
    nixfmt.enable = true;
    prettier.enable = true;
    rustfmt.enable = true;
  };
}
