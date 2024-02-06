const SHELL_SCRIPT: &str = r#"
#!/bin/bash

# Define a function to capture and forward commands to nix-recorder
record_command() {
    echo "$@" >> /tmp/nix-recorder/command_log
    # You might want to forward this to nix-recorder directly instead
}

# Change the prompt to use record_command
export PS1='[\u@\h \W] $(record_command "\$ ")\$ '

# Start a new shell to capture commands
exec $SHELL
 

"#;


fn ensure_script_exists() -> std::io::Result<PathBuf> {
    let mut script_path = env::temp_dir();
    script_path.push("monitor_commands.sh");

    // Check if the script already exists
    if !script_path.exists() {
        // Write the script to a file
        fs::write(&script_path, SHELL_SCRIPT)?;
        // Make the script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;
        }
    }

    Ok(script_path)
}
