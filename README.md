# Nix Recorder

## Overview
`nix-recorder` is a command-line utility designed to enhance the management of Nix shell environments. It uniquely allows users to dynamically add packages to their Nix shell environment, record their terminal commands, and then eject these commands along with the package list to a `flake.nix` file for a fully reproducible environment.

## Inspiration / Idea Credit
The idea for this program was inspired by a tweet from Arian van Putten (@ProgrammerDude) and is a personal exploration into developing CLI tools with Rust. [Tweet by Arian van Putten](https://twitter.com/ProgrammerDude/status/1754200297675554941)

## Features
- **Dynamic Package Management**: Seamlessly add packages to your Nix shell on-the-fly.
- **Automatic Command Recording**: Save terminal commands input during the session and automatically integrate them as shell hooks in the ejected `flake.nix`.
- **Eject to Flake**: Export your current environment into a `flake.nix` file, ensuring reproducibility.
- **Enhanced State Management**: Preserve the state of your environment, including packages and terminal commands, for future sessions.

## Usage Example
This section demonstrates how `nix-recorder` can be used to manage a Nix shell environment, dynamically add packages, and eject the session to a `flake.nix` file.

```bash
    [ben@marin:~/nixRecorder/nixRecorder]$ ./target/release/nixRecorder --start
    [ben@marin nixRecorder]$ ./target/release/nixRecorder --package cowsay
    [ben@marin nixRecorder]$ ./target/release/nixRecorder --package htop
    [ben@marin nixRecorder]$ ls
    Cargo.lock  Cargo.toml  flake.nix  LICENSE  README.md  rust-toolchain  scripts  shell.nix  src  target
    [ben@marin nixRecorder]$ ./target/release/nixRecorder --eject
    [ben@marin nixRecorder]$ ^C
    exit
    [ben@marin nixRecorder]$
    exit
    [ben@marin nixRecorder]$
    exit
```

Upon ejecting the session to a `flake.nix` file, `nix-recorder` records the session's packages and commands:
Notice how the `ls` command is in the shellHook, any commands you enter after starting nix-recorder will be in the shellHook.

   ```nix
    {
      inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
      };

      outputs = { self, nixpkgs, flake-utils }:
        flake-utils.lib.eachDefaultSystem (system:
          let
            pkgs = import nixpkgs {
              inherit system;
            };
          in
          {
              devShells.default = pkgs.mkShell {
                buildInputs = [ pkgs.cowsay pkgs.htop ];
                shellHook = ''
                  ./target/release/nixRecorder --eject
    ./target/release/nixRecorder --package htop
    ./target/release/nixRecorder --package cowsay
    return 0
    unset NIX_ENFORCE_PURITY
    shopt -u nullglob
    unset TZ
    shopt -s execfail
    ./target/release/nixRecorder --package cowsay
    return 0
    unset NIX_ENFORCE_PURITY
    shopt -u nullglob
    unset TZ
    shopt -s execfail
    ./target/release/nixRecorder --package htop
    return 0
    unset NIX_ENFORCE_PURITY
    shopt -u nullglob
    unset TZ
    shopt -s execfail
    ls --color=tty
    ./target/release/nixRecorder --eject
                '';
            };
        });
    }
    ```

To enter the development shell after ejecting, use:

```bash
    $ nix develop
```

## Roadmap
- [x] Save any terminal commands you input and add them as a ShellHook to the ejected Flake.
- [ ] Improve state management and user experience.
- [ ] Support for additional Nix configurations and customization options.
- [ ] **Remove `nix-recorder`-related commands from the shell hook in the ejected `flake.nix` to streamline the development environment.**

## Installation
To install `nix-recorder`, follow these steps:

```bash
    git clone <repository-url>
    cd nix-recorder
    cargo build --release
```
## Contributing 
contributions encouraged! Feel free to submit pull requests or open issues for bugs, feature requests, or enhancements.

## License
GPLv3 (RMS)

