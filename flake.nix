{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-23.05";
    rust-overlay = {
          url = "github:oxalica/rust-overlay";
          inputs.nixpkgs.follows = "nixpkgs";
          inputs.flake-utils.follows = "flake-utils";
    };
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.77.0";
          packageFun = import ./Cargo.nix;
        };

        # The workspace defines a development shell with all of the dependencies
        # and environment settings necessary for a regular `cargo build`
        workspaceShell = rustPkgs.workspaceShell {
          # This adds cargo2nix to the project shell via the cargo2nix flake
          packages = [ cargo2nix.packages."${system}".cargo2nix ];
        };

      in rec {
        # this is the output (recursive) set (expressed for each system)

        devShells = {
          default = workspaceShell; # nix develop
        };

        # the packages in `nix build .#packages.<system>.<name>`
        packages = {
          # nix build .#dsp-meta-cmd
          # nix build .#packages.x86_64-linux.dsp-meta-cmd
          dsp-meta-cmd = (rustPkgs.workspace.dsp-meta-cmd {}).bin;
          # nix build
          default = packages.dsp-meta-cmd;
        };
      }
    );
}
