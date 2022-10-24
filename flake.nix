{
  inputs.easy.url = "github:jooooscha/easy-flake";
  outputs = { easy, ... }:
    easy.rust.env {
      ssl = true;
      name = "syncthing_status";
      root = ./.;
      gdb = true;
    };
}
