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
        // Read commands from the command log file
        let command_log_path = PathBuf::from(".nix_shell_tool_command_log");
        let commands = read_commands(&command_log_path);

        if packages.is_empty() && commands.is_empty() {
            println!("No packages or Commands have been recorded, Ejecting empty Flake");
            let flake_content = generate_flake_nix(&[], &[]);
            fs::write("flake.nix", flake_content).expect("Unable to write flake.nix file");
            std::process::exit(0);
        } else {
            // Include both packages and commands in the flake.nix
            let flake_content = generate_flake_nix(&packages, &commands);
            fs::write("flake.nix", flake_content).expect("Unable to write flake.nix file");
            
            fs::remove_file(state_file).expect("Unable to clear the state file");
            // Optionally, remove the command log file after ejecting to clean up
            fs::remove_file(command_log_path).expect("Unable to clear the command log file");
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

fn read_commands(path: &PathBuf) -> Vec<String> {
    if path.exists() {
        fs::read_to_string(path)
            .expect("Unable to read command log file")
            .lines()
            .map(str::to_string)
            .collect()
    } else {
        Vec::new()
    }
}


fn start_nix_shell(packages: &[String]) {
    //let command_log_path = "./.nix_shell_tool_command_log"; // Adjust as necessary
    let monitor_script_path = "./scripts/monitor_commands.sh"; // Ensure this is the correct path to your script

    let nix_shell_cmd = format!(
        r#"nix-shell -E "{{ pkgs ? import <nixpkgs> {{}} }}: pkgs.mkShell {{
            buildInputs = with pkgs; [ {} ];
            shellHook = '' 
                source {}
            '';}}""#,
        packages.join(" "),
        monitor_script_path
        );

    let mut child = std::process::Command::new("sh")
        .arg("-c")
        .arg(&nix_shell_cmd)
        .spawn()
        .expect("Failed to start nix shell");

    let _ = child.wait().expect("Failed to wait on nix shell");
    return;
}

//fn init_nix_shell() {
//    println!("Starting a basic nix shell...");
//    let mut child = std::process::Command::new("nix-shell")
//        .spawn()
//        .expect("Failed to start nix shell");
//
//    let _ = child.wait().expect("Failed to wait on nix shell");
//}

fn save_state(packages: &[String], path: &PathBuf) {
    let content = packages.join("\n");
    fs::write(path, content).expect("Unable to write state file");
}


fn generate_flake_nix(packages: &[String], commands: &[String]) -> String {
    // Base structure of flake.nix with placeholders for packages and commands
    let flake_nix_template = r#"
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
            buildInputs = [ {packages} ];
            shellHook = ''
              {commands}
            '';
        };
    };);
}
"#;

    // Convert packages and commands into strings suitable for inclusion in the flake
    let packages_str = packages.iter().map(|p| format!("pkgs.{}", p)).collect::<Vec<_>>().join(" ");
    let commands_str = commands.join("\n");

    // Replace placeholders in the template with actual package and command strings
    let flake_nix = flake_nix_template.replace("{packages}", &packages_str).replace("{commands}", &commands_str);

    flake_nix
}

