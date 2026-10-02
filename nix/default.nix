{ rustPlatform, ... }:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "ddnnf-benchmark";
  version = "0.1.0";

  src = ./..;

  cargoHash = "sha256-PCRrgtR9ZZqvxlGQ1LLPjfYKhMyGmJJsvJgkrYG//Ow=";
})
