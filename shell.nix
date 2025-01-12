{
  pimalaya ? import (fetchTarball "https://github.com/pimalaya/nix/archive/master.tar.gz"),
  ...
}@args:

let
  shell = {
    rustToolchainFile = ./rust-toolchain.toml;
  };
in

pimalaya.mkShell (shell // removeAttrs args [ "pimalaya" ])
