use structopt::StructOpt;
use std::{fs, path::PathBuf, env, process::Command};


#[derive(StructOpt, Debug)]
#[structopt(name = "nix-shell-tool")]
struct Opt {
    #[structopt(long)]
    package: Option<String>,
    // / eject to flake.nix and clear the state
    #[structopt(long)]
    eject: bool,

    #[structopt(long)]
    start: bool,
}

fn main() {

    let opt = Opt::from_args();
    let state_file = PathBuf::from(".nix_shell_tool_state");

    // read existing state
    let mut packages = read_state(&state_file);

    if opt.start {
        start_nix_shell(&packages);
    }

    if let Some(package) = opt.package {
        // add package to the list and save state
        packages.push(package);
        save_state(&packages, &state_file);

        // exit the current shell 
        let exit_shell_cmd = "exit";

        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(&exit_shell_cmd)
            .spawn()
            .expect("Failed to stop nix shell");

        let _ = child.wait().expect("Failed to exit nix-shell");

        // restart nix shell with the new package list
        start_nix_shell(&packages);
    }

    if opt.eject {
        //check to see if there exists any packages
        if packages.is_empty(){
            println!("No packages or Commands have been recorded, Ejecting empty Flake");
            let flake_content = generate_flake_nix(&[]);
            fs::write("flake.nix", flake_content).expect("Unable to write flake.nix file");
            //fs::remove_file(state_file).expect("Unable to clear the state file");
            std::process::exit(0);
        }

        else {
            // get flake.nix content and write to a file
            let flake_content = generate_flake_nix(&packages);
            fs::write("flake.nix", flake_content).expect("Unable to write flake.nix file");
            
            fs::remove_file(state_file).expect("Unable to clear the state file");
            // exit out of the current nix shell
            std::process::exit(0);
        }
    }
}

fn read_state(path: &PathBuf) -> Vec<String> {
    if path.exists() {
        fs::read_to_string(path)
            .expect("Unable to read state file")
            .lines()
            .map(str::to_string)
            .collect()
    } else {
        Vec::new()
    }
}

fn start_nix_shell(packages: &[String]) {
    if packages.is_empty() {
        println!("Starting a basic nix shell...");
        init_nix_shell();
    } else {
        // Construct the command to start the nix shell with the packages
        let nix_shell_cmd = format!("nix-shell -p {}", packages.join(" "));
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(&nix_shell_cmd)
            .spawn()
            .expect("Failed to start nix shell");

        let _ = child.wait().expect("Failed to wait on nix shell");
    }
}

fn init_nix_shell() {
    println!("Starting a basic nix shell...");
    let mut child = std::process::Command::new("nix-shell")
        .spawn()
        .expect("Failed to start nix shell");

    let _ = child.wait().expect("Failed to wait on nix shell");
}

fn save_state(packages: &[String], path: &PathBuf) {
    let content = packages.join("\n");
    fs::write(path, content).expect("Unable to write state file");
}

fn generate_flake_nix(packages: &[String]) -> String {
    // Base structure of flake.nix
    let mut flake_nix = r#"
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
          buildInputs = ["#.to_string();

    // Add packages to the buildInputs array
    for package in packages {
        flake_nix.push_str(&format!("pkgs.{} ", package));
    }

    // Close the buildInputs array and the rest of the structure
    flake_nix.push_str(r#"];
        };
      }
    );
}
"#);

    flake_nix
}

