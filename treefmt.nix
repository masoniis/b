{ ... }:
{
  projectRootFile = "flake.nix";
  programs = {
    clang-format.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
    rustfmt.enable = true;
  };
}
