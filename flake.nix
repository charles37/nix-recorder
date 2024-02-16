
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
          devShells.default = pkgs.mkShell {
            buildInputs = [ pkgs.cowsay pkgs.htop ];
            shellHook = ''
              return 0
unset NIX_ENFORCE_PURITY
shopt -u nullglob
unset TZ
shopt -s execfail
ls --color=tty
./target/debug/nixRecorder --package cowsay
return 0
unset NIX_ENFORCE_PURITY
shopt -u nullglob
unset TZ
shopt -s execfail
./target/debug/nixRecorder --package htop
return 0
unset NIX_ENFORCE_PURITY
shopt -u nullglob
unset TZ
shopt -s execfail
./target/debug/nixRecorder --eject
            '';
  };
}
