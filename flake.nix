{
    description = "Rust dev env";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs =
        { self, nixpkgs, ... }@inputs:
        let
            system = "x86_64-linux";
            pkgs = import nixpkgs {
                inherit system;
                config.allowUnfree = true;
            };
        in
            {
            devShells."x86_64-linux".default = pkgs.mkShell {
                packages = with pkgs; [
                    zsh
                    cargo
                    rustc
                    clippy
                    rustfmt
                    rust-analyzer
                    pkg-config
                    udev
                ];
            };
        };
}
