{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      perSystem = { pkgs, config, lib, ... }:
        let
          runtimeLibraryPath = lib.makeLibraryPath [
            pkgs.openssl
            pkgs.sqlite
            pkgs.libpq
            pkgs.zlib
          ];

          rustRuntimeShellHook = lib.optionalString pkgs.stdenv.isLinux ''
            export RUSTFLAGS="''${RUSTFLAGS:-} -C link-arg=-Wl,-rpath,${runtimeLibraryPath}"
          '';

          initDb = pkgs.writeShellApplication {
            name = "init-db";
            runtimeInputs = [
              pkgs.coreutils
              pkgs.sqlx-cli
              pkgs.diesel-cli
            ];
            text = ''
              set -euo pipefail

              if [ ! -d "$PWD/basics/todo/migrations" ] || [ ! -d "$PWD/databases/diesel/migrations" ]; then
                echo "init-db must be run from the repository root" >&2
                exit 1
              fi

              DATABASE_URL="sqlite://./todo.db" sqlx database create
              chmod a+rwx ./todo.db
              DATABASE_URL="sqlite://./todo.db" sqlx migrate run --source=./basics/todo/migrations

              (
                cd databases/diesel
                DATABASE_URL=test.db diesel migration run
                chmod a+rwx ./test.db
              )
            '';
          };
        in
        {
          formatter = pkgs.nixpkgs-fmt;

          devShells.default = pkgs.mkShell {
            packages = [
              config.formatter
              pkgs.nodePackages.prettier
              pkgs.taplo
              pkgs.fd
              pkgs.just
              pkgs.pkg-config
              pkgs.openssl
              pkgs.sqlite
              pkgs.libpq
              pkgs.redis
              pkgs.sqlx-cli
              pkgs.diesel-cli
            ] ++ lib.optional pkgs.stdenv.isDarwin [
              pkgs.pkgsBuildHost.libiconv
            ];
            shellHook = rustRuntimeShellHook;
          };

          devShells.ci = pkgs.mkShell {
            packages = [
              pkgs.just
              pkgs.pkg-config
              pkgs.openssl
              pkgs.sqlite
              pkgs.libpq
              pkgs.redis
              pkgs.sqlx-cli
              pkgs.diesel-cli
            ] ++ lib.optional pkgs.stdenv.isDarwin [
              pkgs.pkgsBuildHost.libiconv
            ];
            shellHook = rustRuntimeShellHook;
          };

          apps.init-db = {
            type = "app";
            program = lib.getExe initDb;
          };
        };
    };
}
