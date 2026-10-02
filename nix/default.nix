{
  fenix,
  makeRustPlatform,
  stdenv,
  ...
}:
let
  toolchain = fenix.packages.${stdenv.system}.stable.minimalToolchain;
in
(makeRustPlatform {
  cargo = toolchain;
  rustc = toolchain;
}).buildRustPackage
  (finalAttrs: {
    pname = "ddnnf-benchmark";
    version = "0.1.0";

    src = ./..;
    cargoLock = {
      lockFile = ../Cargo.lock;
      allowBuiltinFetchGit = true;
    };
  })
