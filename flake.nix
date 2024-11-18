{
  description = "PureRng";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    
    # old nixpkgs version for random-quality
    nixpkgs-random = {
      url = "github:NixOS/nixpkgs?rev=48723f48ab92381f0afd50143f38e45cf3080405";
      flake = false;
    };
    random-quality = {
      url = "github:tweag/random-quality";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, nixpkgs-random, random-quality }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forEachSupportedSystem = f:
        nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      devShells = forEachSupportedSystem (system:
        let
          randomQualityOverlay = final: prev: {
            dieharder = final.callPackage "${random-quality}/nix/dieharder" {};
            gjrand = final.callPackage "${random-quality}/nix/gjrand" {};
            practrand = final.callPackage "${random-quality}/nix/PractRand" {};
            rademacher-fpl-test = final.callPackage "${random-quality}/nix/rademacher-fpl-test" {};
            testu01 = final.callPackage "${random-quality}/nix/TestU01" {};
            testu01-stdin = final.callPackage "${random-quality}/testu01-stdin" {};
          };

          pkgs = import nixpkgs { inherit system; };

          randomQualityPkgs = import nixpkgs-random {
            inherit system;
            overlays = [ randomQualityOverlay ];
          };
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustc
              cargo
              rust-analyzer
              rustfmt
              lldb
              clippy
              bacon

              python3
              python3Packages.matplotlib
              pkgs.graphviz
            ];
          };

          quality_tests = pkgs.mkShell {
            buildInputs = with randomQualityPkgs; [
              dieharder
              gjrand
              practrand
              rademacher-fpl-test
              testu01
              testu01-stdin
            ];
          };
        });
    };
}
