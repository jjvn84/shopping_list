{
  description = "Entorno de desarrollo básico para Rust con rust-analyzer y clippy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          pkg-config

          # System Libraries
          cairo
          glib
          gtk3
          libsoup_3
          openssl
          webkitgtk_4_1
          xdotool
        ];
        # OpenSSL Environment Variables
        OPENSSL_DIR = "${pkgs.openssl.dev}";
        OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.glib.dev}/lib/pkgconfig:${pkgs.cairo.dev}/lib/pkgconfig:${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.libsoup_3.dev}/lib/libsoup-3.0.pc:${pkgs.webkitgtk_4_1.dev}/lib/pkgconfig:${pkgs.xdotool}/lib/pkgconfig";
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };
    };
}
