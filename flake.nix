{
  description = "Entorno de desarrollo básico para Rust con rust-analyzer y clippy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
        config = {
          allowUnfree = true;
          android_sdk = {
            accept_license = true;
          };
        };
      };

      # 3. Add Emulator and System Image to your environment
      androidEnv = pkgs.androidenv.composeAndroidPackages {
        includeNDK = true;
        ndkVersions = [ "27.3.13750724" ];
        platformVersions = [ "34" ];
        buildToolsVersions = [ "34.0.0" ];

        includeEmulator = false;
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "rust-analyzer"
          "clippy"
        ];
        targets = [
          "x86_64-unknown-linux-gnu"
          "x86_64-linux-android"
        ];
      };

    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          rustToolchain
          pkgs.pkg-config
          pkgs.cairo
          pkgs.glib
          pkgs.gtk3
          pkgs.libsoup_3
          pkgs.openssl
          pkgs.webkitgtk_4_1
          pkgs.xdotool

          # Android Development Tools
          androidEnv.androidsdk
          pkgs.jdk17
          pkgs.gradle

        ];

        OPENSSL_DIR = "${pkgs.openssl.dev}";
        OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.glib.dev}/lib/pkgconfig:${pkgs.cairo.dev}/lib/pkgconfig:${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.libsoup_3.dev}/lib/libsoup-3.0.pc:${pkgs.webkitgtk_4_1.dev}/lib/pkgconfig:${pkgs.xdotool}/lib/pkgconfig";
        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

        ANDROID_HOME = "${androidEnv.androidsdk}/libexec/android-sdk";
        ANDROID_SDK_ROOT = "${androidEnv.androidsdk}/libexec/android-sdk";
        ANDROID_NDK_ROOT = "${androidEnv.androidsdk}/libexec/android-sdk/ndk/27.3.13750724";
        ANDROID_NDK_HOME = "${androidEnv.androidsdk}/libexec/android-sdk/ndk/27.3.13750724";
        JAVA_HOME = "${pkgs.jdk17.home}";

        CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER = "${androidEnv.androidsdk}/libexec/android-sdk/ndk/27.3.13750724/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android27-clang";
      };
    };
}
