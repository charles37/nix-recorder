# Nix Recorder

## Overview
`nix-recorder` is a command-line utility designed to enhance the management of Nix shell environments. It uniquely allows users to dynamically add packages to their Nix shell environment, record their terminal commands, and then eject these commands along with the package list to a `flake.nix` file for a fully reproducible environment.

## Inspiration / Idea Credit
The idea for this program was inspired by a tweet from Arian van Putten (@ProgrammerDude) and is an exploration into developing CLI tools with Rust. [Tweet by Arian van Putten](https://twitter.com/ProgrammerDude/status/1754200297675554941)

## Features
- **Dynamic Package Management**: Seamlessly add packages to your Nix shell on-the-fly.
- **Automatic Command Recording**: Save terminal commands input during the session and automatically integrate them as shell hooks in the ejected `flake.nix`.
- **Eject to Flake**: Export your current environment into a `flake.nix` file, ensuring reproducibility.
- **Enhanced State Management**: Preserve the state of your environment, including packages and terminal commands, for future sessions.

## Roadmap
- [x] Save any terminal commands you input and add them as a ShellHook to the ejected Flake.
- [ ] Improve state management and user experience.
- [ ] Support for additional Nix configurations and customization options.

## Installation
To install `nix-recorder`, follow these steps:


```bash
    git clone https://github.com/charles37/nix-recorder.git
    cd nix-recorder
    cargo build --release
```

## Usage

### Starting a Nix Shell
Initiate a Nix shell session with the saved state or a basic environment:

```bash
    nix-recorder --start
```

### Adding a Package
Incorporate a package into your current Nix shell session dynamically:

```bash
    nix-recorder --package <package_name>
```

The shell will restart with the newly added package.

### Recording Commands
Simply use the tool as demonstrated; it automatically records your terminal commands for the session.

### Ejecting to Flake.nix
Generate a `flake.nix` file reflecting your session's packages and commands:

```bash
    nix-recorder --eject
```

## Requirements
- Rust programming language
- Nix package manager

## Contributing
contributions are encouraged! Feel free to submit pull requests or open issues for bugs, feature requests, or enhancements.

## License
GPLv3 (RMS)

