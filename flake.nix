{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      # Setup packages
      system = "x86_64-linux";
      overlays = [ rust-overlay.overlays.default ];
      pkgs = import nixpkgs { inherit system overlays; };

      # Rust toolchain with cross compilation support
      rust = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" ];
      };

      # System dependencies
      buildInputs = with pkgs; [
        rust
        libfprint
      ];
    in
    {
      devShells.${system} = {
        default = pkgs.mkShell {
          inherit buildInputs;
          shellHook = with pkgs; ''
            export LD_LIBRARY_PATH="${lib.makeLibraryPath buildInputs}"
            export LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib"
          '';
        };
      };
    };
}
