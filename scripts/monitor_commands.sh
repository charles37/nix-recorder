#!/usr/bin/env bash

# This script is now used to initialize command monitoring in a new shell session.
# It will set the command prompt to capture and log commands.

export PS1='[\u@\h \W]\$ '
trap 'record_command "$BASH_COMMAND"' DEBUG

record_command() {
    local cmd="$1"
    if [[ "$cmd" != "record_command "* ]] && [[ -n $cmd ]]; then
        echo "$cmd" >> .nix_shell_tool_command_log
    fi
}


