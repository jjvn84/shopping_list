{
  description = "Entorno Jupyter con uv y automatización de librerías mediante nix-ld";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
        ];

        # Tell rust-analyzer where to find the standard library source
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

      };
    };
}
