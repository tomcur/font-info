{ lib
, rustPlatform
, pkg-config
, fontconfig
}:
rustPlatform.buildRustPackage {
  pname = "font-info";
  version = (lib.trivial.importTOML ./Cargo.toml).package.version;
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    fontconfig
  ];
}
