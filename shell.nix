{ pkgs ? import <nixpkgs> { } }:

let inherit (pkgs) stdenv;

in pkgs.mkShell {
  name = "development";

  buildInputs = with pkgs; [ pkg-config udev protobuf cargo just ];
  LOCALE_ARCHIVE = "${pkgs.glibcLocales}/lib/locale/locale-archive";

  shellHook = ''
    rustup target add armv7-unknown-linux-gnueabihf
  '';
}
