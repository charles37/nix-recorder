# Nix Recorder

## Overview
`nix-recorder` is a command-line utility designed to manage Nix shell environments. It allows users to dynamically add packages to their Nix shell environment, save the current state, and eject to a `flake.nix` file for reproducible environments.

## Inspiration / Idea Credit
The Idea for this program came from a Tweet[https://twitter.com/ProgrammerDude/status/1754200297675554941] by Arian van Putten (@ProgrammerDude[https://twitter.com/ProgrammerDude]) and is mostly an experiment for programming CLI Tools in Rust

## Features
- **Dynamic Package Management**: Add packages to your Nix shell on-the-fly.
- **Start Nix Shell**: Initialize a basic or package-enhanced Nix shell.
- **Eject to Flake**: Convert your current environment into a `flake.nix` file, making it reproducible.
- **State Management**: Save the current state of your environment for future sessions.

## Roadmap
= Save any terminal commands you input and add them as a ShellHook to the ejected Flake
- Imporve state management

## Installation
To install `nix-recorder`, clone the repository and build the project using Cargo (Rust's package manager):

```bash
git clone <repository-url>
cd nix-recorder
cargo build --release
```

## Usage

### Starting a Nix Shell
To start a Nix shell with the currently saved state:
```bash
nix-recorder --start
```

### Adding a Package
To add a package to your current Nix shell session:
```bash
nix-recorder --package <package_name>
```
This command will restart the Nix shell with the new package included.

### Ejecting to Flake.nix
To create a `flake.nix` file based on the current session:
```bash
nix-recorder --eject
```

## Requirements
- Rust programming language
- Nix package manager

## Contributing
Contributions to `nix-recorder` are welcome. Please submit a pull request or open an issue for bugs, features, or enhancements.

## License
Specify your licensing information here.

