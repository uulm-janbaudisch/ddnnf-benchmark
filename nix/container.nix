{ dockerTools, ddnnf-benchmark, ... }:
dockerTools.buildLayeredImage {
  name = "ddnnife-benchmark";
  contents = [
    ddnnf-benchmark
  ];
  config = {
    Entrypoint = [ "/bin/ddnnife" ];
  };
}
