{
  description = "Aletheia - RSR Compliance Verification Tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustVersion = pkgs.rust-bin.stable."1.75.0".default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
          targets = [ "x86_64-unknown-linux-gnu" "x86_64-unknown-linux-musl" ];
        };

        # Build the aletheia binary
        aletheia = pkgs.rustPlatform.buildRustPackage {
          pname = "aletheia";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # Zero dependencies - no native build inputs needed
          nativeBuildInputs = [ ];
          buildInputs = [ ];

          # Run tests during build
          doCheck = true;
          checkPhase = ''
            cargo test --release
          '';

          meta = with pkgs.lib; {
            description = "RSR compliance verification tool - truth in repository standards";
            homepage = "https://gitlab.com/maa-framework/6-the-foundation/aletheia";
            license = with licenses; [ mit ];
            maintainers = [ ];
            platforms = platforms.unix;
            mainProgram = "aletheia";
          };
        };

      in {
        # Package output
        packages = {
          default = aletheia;
          aletheia = aletheia;
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustVersion
            pkgs.just
            pkgs.cargo-audit
            pkgs.cargo-deny
          ];

          shellHook = ''
            echo "Aletheia development environment loaded"
            echo "Rust version: $(rustc --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo test     - Run tests"
            echo "  cargo run      - Run self-verification"
            echo "  just check     - Run all checks"
            echo ""
          '';
        };

        # Apps for nix run
        apps = {
          default = {
            type = "app";
            program = "${aletheia}/bin/aletheia";
          };
        };

        # Checks for CI
        checks = {
          inherit aletheia;

          # Format check
          fmt = pkgs.runCommand "check-fmt" {
            buildInputs = [ rustVersion ];
          } ''
            cd ${./.}
            cargo fmt --check
            touch $out
          '';

          # Clippy check
          clippy = pkgs.runCommand "check-clippy" {
            buildInputs = [ rustVersion ];
          } ''
            cd ${./.}
            cargo clippy -- -D warnings
            touch $out
          '';
        };
      }
    );
}
