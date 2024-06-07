#!/usr/bin/env bash


export PS1='[\u@\h \W]\$ '
trap 'record_command "$BASH_COMMAND"' DEBUG

record_command() {
    local cmd="$1"
    if [[ "$cmd" != "record_command "* ]] && [[ -n $cmd ]]; then
        echo "$cmd" >> .nix_shell_tool_command_log
    fi
}


