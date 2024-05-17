{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        name = cargoToml.package.name;
      in
      rec
      {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ cargo rustc rust-analyzer rustfmt ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };

        packages = rec {
          "${name}" = pkgs.rustPlatform.buildRustPackage
            {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };
        };
        defaultPackage = packages.${name};

        apps = rec {
          "${name}" = {
            type = "app";
            program = "${self.packages.${system}.${name}}/bin/${name}";
          };
        };
        defaultApp = apps.${name};
      });
}
