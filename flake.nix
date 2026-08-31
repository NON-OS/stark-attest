{
  # The reproducibility base. Three things come out of this flake, all pinned
  # by flake.lock: a dev shell with the Rust and Lean toolchains, the built
  # CLI as a package, and `nix flake check` running the unit tests and the
  # adversarial suite. A stranger with Nix reproduces every claim this repo
  # makes with two commands and no trust in the author's machine.
  description = "stark-attest: one 32-byte statement over a set of artifacts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cli = pkgs.rustPlatform.buildRustPackage {
          pname = "stark-attest";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # the adversarial suite proves real trailers; give it release speed
          checkType = "release";
          cargoTestFlags = [ "--workspace" ];
        };
      in {
        packages.default = cli;

        apps.default = {
          type = "app";
          program = "${cli}/bin/stark-attest";
        };

        checks = {
          build = cli;
          selftest = pkgs.runCommand "stark-attest-selftest" { } ''
            ${cli}/bin/stark-attest selftest
            touch $out
          '';
        };

        devShells.default = pkgs.mkShell {
          name = "stark-attest";
          packages = with pkgs; [
            rustup
            gnumake
            git
            # the machine-checked facts under lean/
            lean4
          ];
          shellHook = ''
            echo "stark-attest shell. cargo test for the suites, lake build in lean/ for the proofs."
          '';
        };
      });
}
