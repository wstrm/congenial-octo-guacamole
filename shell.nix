let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/a58a0b5098f0c2a389ee70eb69422a052982d990.tar.gz")) {};
in pkgs.mkShell {
  buildInputs = [ pkgs.cargo pkgs.rustc ];
}
