{
  description = "A simple flake with a default devShell for x86_64-linux";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = import nixpkgs {system = "x86_64-linux";};
  in {
    devShells.x86_64-linux = {
      default = pkgs.mkShell {
        packages = with pkgs; [
          asciinema
          asciinema-agg
          asciinema-automation

          nodejs
        ];
        shellHook = ''
          export STARSHIP_CONFIG=$(realpath ./starship.toml)
        '';
      };
    };
  };
}
