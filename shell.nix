{ pkgs ? import <nixpkgs> { } }:

let inherit (pkgs) stdenv;

in pkgs.mkShell {
  name = "development";

  buildInputs = with pkgs; [ cargo just ];
  LOCALE_ARCHIVE = "${pkgs.glibcLocales}/lib/locale/locale-archive";

}
