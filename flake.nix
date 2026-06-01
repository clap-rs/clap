{
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { nixpkgs, fenix, ... }:
    let
      pkgs = nixpkgs.legacyPackages.aarch64-darwin;
    in
    {
      devShells.aarch64-darwin.default = pkgs.mkShell {
        shellHook = ''
          export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}";
        '';
        packages = [
          pkgs.lldb
          pkgs.libiconv
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          (fenix.packages.aarch64-darwin.combine [
            fenix.packages.aarch64-darwin.stable.cargo
            fenix.packages.aarch64-darwin.stable.rust
            fenix.packages.aarch64-darwin.targets.wasm32-wasi.stable.rust-std
            fenix.packages.aarch64-darwin.targets.wasm32-wasip1.stable.rust-std
            fenix.packages.aarch64-darwin.targets.aarch64-apple-darwin.stable.rust-std
          ])
        ];
      };

      formatter.aarch64-darwin = pkgs.nixfmt-rfc-style;

    };

}
