{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nitro-util = {
      url = "github:/monzo/aws-nitro-util";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    fenix,
    naersk,
    nitro-util,
  }: let
    system = "x86_64-linux";
    server = import ./server {
      inherit nixpkgs fenix naersk;
    };
    enclave = import ./enclave {
      inherit nixpkgs server nitro-util;
    };
  in {
    formatter.${system} = nixpkgs.legacyPackages.${system}.alejandra;
    packages.${system} = {
      server = server.packages.${system}.default;
      enclave = enclave.packages.${system}.default;
    };
  };
}
