let
  # 1. Pinned nixpkgs commit (NixOS 26.05 stable channel from July 2026)
  nixpkgsCommit = "293d6abedf0478e681a4dfcfcb35b30fc796a32f";
  nixpkgs = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/${nixpkgsCommit}.tar.gz";
  };

  # 2. Pinned rust-overlay commit (oxalica/rust-overlay from July 2026)
  rustOverlayCommit = "afacd6819d3765a05814ee8e3de74c77d42ac799";
  rustOverlay = builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/${rustOverlayCommit}.tar.gz";
  };

  # 3. Import nixpkgs with the rust overlay
  pkgs = import nixpkgs {
    overlays = [ (import rustOverlay) ];
  };

  # 4. Define the Rust toolchain
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [
      "rust-src"       # Required for rust-analyzer auto-complete
      "rust-analyzer"  # Rust language server
      "rustfmt"        # Rust code formatter
      "clippy"         # Rust linter
    ];
  };

in
pkgs.mkShell {
  name = "rust-dev-shell";

  # Packages to install in the development environment
  buildInputs = with pkgs; [
    rustToolchain
    pkg-config
    openssl
    
    # You can add other tools you use here:
    # cargo-watch
    # cargo-edit
    # git
  ];

  # Set RUST_SRC_PATH so rust-analyzer can find standard library source code
  RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

  # Environment variables for dynamic library loading (helpful for Linux/NixOS)
  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}:$LD_LIBRARY_PATH"
    echo "🦀 Pinned Rust Development Environment Loaded! 🦀"
    echo "Rust:   $(rustc --version)"
    echo "Cargo:  $(cargo --version)"
  '';
}
