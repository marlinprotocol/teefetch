{
  nixpkgs,
  fenix,
  naersk,
}: let
  system = "x86_64-linux";
  pkgs = nixpkgs.legacyPackages."${system}";
  target = "x86_64-unknown-linux-musl";
  toolchain = with fenix.packages.${system};
    combine [
      stable.cargo
      stable.rustc
      targets.${target}.stable.rust-std
    ];
  naersk' = naersk.lib.${system}.override {
    cargo = toolchain;
    rustc = toolchain;
  };
in {
  packages.${system}.default = naersk'.buildPackage {
    src = ./.;
    CARGO_BUILD_TARGET = target;
    TARGET_CC = "${pkgs.pkgsMusl.gcc}/bin/cc";
    buildInputs = [
      pkgs.pkgsMusl.gcc
    ];
  };
}
