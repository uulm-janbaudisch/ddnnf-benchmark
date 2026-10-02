{
  description = "Benchmarking tool for d-DNNF computations.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
      ...
    }:
    let
      lib = nixpkgs.lib;

      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      treefmt =
        system:
        (treefmt-nix.lib.evalModule nixpkgs.legacyPackages.${system} ./nix/treefmt.nix).config.build;
    in
    {
      formatter = lib.genAttrs systems (system: (treefmt system).wrapper);
      packages = lib.genAttrs systems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          pkgsSelf = self.packages.${system};
        in
        {
          default = pkgs.callPackage ./nix/default.nix { };
          container = pkgs.callPackage ./nix/container.nix { ddnnf-benchmark = pkgsSelf.default; };
        }
      );
      checks = lib.genAttrs systems (system: {
        format = (treefmt system).check self;
      });
    };
}
