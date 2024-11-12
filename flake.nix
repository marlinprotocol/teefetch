{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    server = {
      url = "path:./server";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    enclave = {
      url = "path:./enclave";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.server.follows = "server";
    };
  };
  outputs = { self, nixpkgs, server, enclave }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
    in
    {
      packages.${system} = {
        server = server.packages.${system}.default;
        enclave = enclave.packages.${system}.default;
      };
    };
}
