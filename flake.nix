{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Explicitly specify the nightly Rust toolchain
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl.dev
            openssl.out
            clang
            llvm
            libclang
            gnumake
          ];

          # Environment variables for build.rs and bindgen
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          CC = "${pkgs.clang}/bin/clang";

          shellHook = ''
            echo "Rust nightly development environment loaded!"
            echo "Build with: cargo build --release"
            echo ""
            echo "Adding ~/.cargo/bin to PATH..."
            export PATH="$HOME/.cargo/bin:$PATH"
            echo ""
            echo "Checking Rust installation..."
            rustc --version
            echo ""
            echo "Checking additional dependencies..."
            clang --version
            llvm-config --version
            make --version
          '';
        };
      }
    );
}
