{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      perSystem = { pkgs, config, inputs', system, lib, ... }: {
        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          packages = [
            config.formatter
            pkgs.nodePackages.prettier
            pkgs.taplo
            pkgs.just
          ] ++ lib.optional pkgs.stdenv.isDarwin [
            pkgs.pkgsBuildHost.libiconv
            pkgs.pkgsBuildHost.darwin.apple_sdk.frameworks.Security
            pkgs.pkgsBuildHost.darwin.apple_sdk.frameworks.CoreFoundation
            pkgs.pkgsBuildHost.darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        };
      };
    };
}
