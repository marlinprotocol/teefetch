{
  nixpkgs,
  server,
  nitro-util,
}: let
  system = "x86_64-linux";
  pkgs = nixpkgs.legacyPackages."${system}";
  nitro = nitro-util.lib.${system};
  eifArch = "x86_64";
  supervisord = pkgs.fetchurl {
    url = "https://artifacts.marlin.org/oyster/binaries/supervisord_c2cae38b_linux_amd64";
    sha256 = "46bf15be56a4cac3787f3118d5b657187ee3e4d0a36f3aa2970f3ad3bd9f2712";
  };
  keygenEd25519 = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/binaries/keygen-ed25519_v1.0.0_linux_amd64";
    sha256 = "e68c55cab8ff21de5b9c9ab831b3365717cceddf5f0ad82fee57d1ef40231d3c";
  };
  keygenSecp256k1 = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/binaries/keygen-secp256k1_v1.0.0_linux_amd64";
    sha256 = "9d4344e491413abb559e507ccfcd4397edf736199fb1a1a39c9ae9c576655579";
  };
  itvtProxy = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/binaries/ip-to-vsock-transparent_v1.0.0_linux_amd64";
    sha256 = "15ecdf4ed7c0a3f65ebfa2fb10f0c1cb60e67677162db8cca6915aabb5afd4b9";
  };
  vtiProxy = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/binaries/vsock-to-ip_v1.0.0_linux_amd64";
    sha256 = "8ad67e28b18a742c3b94078954021215b57a287ee634f09556efabcac0b99597";
  };
  attestationServer = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/binaries/attestation-server_v2.0.0_linux_amd64";
    sha256 = "b05852fa4ebda4d9a88ab2b61deae5f22b7026f4d99c5eeeca3c31ee99a77a71";
  };
  dnsproxy = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/binaries/dnsproxy_v0.72.0_linux_amd64";
    sha256 = "1c2bc5eab0dcdbac89c0ef6515e328227de9987af618a7138cc05d9bc53590c1";
  };
  kernel = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/kernels/vanilla_7614f199_amd64/bzImage";
    sha256 = "16a90b65a2920f51462f4e4a71217efd5b7fc63b93bd72a2ad3c759160d472ab";
  };
  kernelConfig = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/kernels/vanilla_7614f199_amd64/bzImage.config";
    sha256 = "fab17e49df1b621dfe8584ede8124712116b24c6d3b61cd91dc209ddf7da2b2c";
  };
  nsmKo = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/kernels/vanilla_7614f199_amd64/nsm.ko";
    sha256 = "42b49249abe01a1d32639bf1011e62418ac10b0360328138ea36271451c3a587";
  };
  init = builtins.fetchurl {
    url = "https://artifacts.marlin.org/oyster/kernels/vanilla_7614f199_amd64/init";
    sha256 = "847bac1648acedc01a76f0e0108d3f08df956ed267622a51066fd9e1d8a29ee8";
  };
  setup = ./. + "/setup.sh";
  supervisorConf = ./. + "/supervisord.conf";
  server' = server.packages.${system}.default;
  app = pkgs.runCommand "app" {} ''
    echo Preparing the app folder
    pwd
    mkdir -p $out
    mkdir -p $out/app
    mkdir -p $out/etc
    cp ${supervisord} $out/app/supervisord
    cp ${keygenEd25519} $out/app/keygen-ed25519
    cp ${keygenSecp256k1} $out/app/keygen-secp256k1
    cp ${itvtProxy} $out/app/ip-to-vsock-transparent
    cp ${vtiProxy} $out/app/vsock-to-ip
    cp ${attestationServer} $out/app/attestation-server
    cp ${dnsproxy} $out/app/dnsproxy
    cp ${setup} $out/app/setup.sh
    echo ${server'}
    cp ${server'}/bin/server $out/app/teefetch
    chmod +x $out/app/*
    cp ${supervisorConf} $out/etc/supervisord.conf
  '';
  # kinda hacky, my nix-fu is not great, figure out a better way
  initPerms = pkgs.runCommand "initPerms" {} ''
    cp ${init} $out
    chmod +x $out
  '';
in {
  packages.${system}.default = nitro.buildEif {
    name = "enclave";
    arch = eifArch;

    init = initPerms;
    kernel = kernel;
    kernelConfig = kernelConfig;
    nsmKo = nsmKo;
    cmdline = builtins.readFile nitro.blobs.${eifArch}.cmdLine;

    entrypoint = "/app/setup.sh";
    env = "";
    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [app pkgs.busybox pkgs.nettools pkgs.iproute2 pkgs.iptables-legacy pkgs.cacert];
      pathsToLink = ["/bin" "/app" "/etc"];
    };
  };
}
