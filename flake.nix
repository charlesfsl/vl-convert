{
  description = "vl-convert segfault local reproduction";

  inputs = {
    nixpkgs = { url = "github:NixOS/nixpkgs/nixos-24.11"; };
    flake-utils = { url = "github:numtide/flake-utils"; };
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
  let
    inherit (nixpkgs.lib) optional optionals;
    pkgs = import nixpkgs { inherit system; };


    locales = pkgs.glibcLocales;
  in
  with pkgs;
  {
    devShells.default = pkgs.mkShell {
      buildInputs = [
        locales
        pkgs.rustup
        pkgs.cargo
        pkgs.lldb
      ] ++ optional stdenv.isLinux inotify-tools
      ++ optional stdenv.isDarwin terminal-notifier
      ++ optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
        CoreFoundation
        CoreServices
      ]);
    };
    packages.default = cargo;
  });
}
