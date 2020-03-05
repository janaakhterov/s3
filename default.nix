with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "S3";

  src = null;

  buildInputs = [
    # rust
    # openssl
    git
    stdenv
    fish
  ];
}
