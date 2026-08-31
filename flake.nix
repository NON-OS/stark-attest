{
  # The reproducibility base.
  #
  #   nix build            the CLI as a derivation, from pinned inputs
  #   nix flake check      build, unit tests, adversarial suite, selftest,
  #                        and a lake build of the machine-checked proofs
  #   nix develop          a shell with the Rust and Lean toolchains
  #   nix run . -- verify  the tool itself
  #
  # Everything a stranger needs to re-establish this repository's claims
  # without trusting the author's machine, pinned by flake.lock. The proofs
  # are a check like any other: a repository that claims machine-checked
  # facts should fail its own gate when they stop checking.
  description = "stark-attest: one 32-byte statement over a set of artifacts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        # The CLI, built from this tree with the lock file's exact dependency
        # graph. Tests run in release: the adversarial suite proves real
        # trailers, and at debug speed that is minutes of nothing useful.
        stark-attest = pkgs.rustPlatform.buildRustPackage {
          pname = "stark-attest";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          checkType = "release";
          cargoTestFlags = [ "--workspace" ];
          meta = {
            description = "Attest a set of artifacts with one 32-byte statement";
            license = pkgs.lib.licenses.agpl3Plus;
            mainProgram = "stark-attest";
          };
        };

        # The Lean development, checked by lake against the pinned toolchain.
        # Offline: lake is given the source tree and no network, so a green
        # result means the proofs check, not that a download succeeded.
        proofs = pkgs.stdenv.mkDerivation {
          name = "stark-attest-proofs";
          src = ./lean;
          nativeBuildInputs = [ pkgs.lean4 ];
          buildPhase = ''
            export HOME=$TMPDIR
            lake build
          '';
          installPhase = "touch $out";
        };
      in {
        packages = {
          default = stark-attest;
          inherit stark-attest proofs;
        };

        apps.default = {
          type = "app";
          program = "${stark-attest}/bin/stark-attest";
        };

        checks = {
          # the package builds and its whole test suite passes
          build = stark-attest;
          # the machine-checked facts still check
          inherit proofs;
          # the tool refuses what it must refuse
          selftest = pkgs.runCommand "stark-attest-selftest" { } ''
            ${stark-attest}/bin/stark-attest selftest
            touch $out
          '';
          # formatting is part of the contract, not a preference
          fmt = pkgs.runCommand "stark-attest-fmt" {
            nativeBuildInputs = [ pkgs.rustfmt pkgs.cargo ];
          } ''
            cd ${self}
            cargo fmt --all -- --check
            touch $out
          '';
        };

        devShells.default = pkgs.mkShell {
          name = "stark-attest";
          packages = with pkgs; [
            rustup
            rustfmt
            lean4
            gnumake
            git
          ];
          shellHook = ''
            echo "stark-attest: cargo test for the suites, lake build in lean/ for the proofs."
          '';
        };
      });
}
