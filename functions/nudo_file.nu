#!/usr/bin/env nu

# ========== Load Function Modules ==========
const functions_dir = ($nu.default-config-dir | path join "functions")
use ($functions_dir | path join "editsu.nu") * 
use ($functions_dir | path join "gpu-mode.nu") *
use ($functions_dir | path join "pkg_manager.nu") *
use ($functions_dir | path join "bluecon.nu") *
use ($functions_dir | path join "settings.nu") *
use ($functions_dir | path join "utils.nu") *
use ($functions_dir | path join "autostart.nu") *
# ========== HELP PAGE ==========
def help_command [] {
    print ""
    print $"(ansi blue)nudo — Administrative + package management wrapper(ansi reset)"
    print "---------------------------------------------------------"
    print $"(ansi red)Usage:(ansi reset) nudo <command> [args]\n"

    let cmds = [
        ["edit <path>",                 "Edit config file (root)"],
        ["install <pkg...>",            "Install packages"],
        ["remove <pkg/env/toggle>",     "Remove pkg or env/toggle"],
	["remove autostart <cmd>", 	"Remove an autostart"],
        ["update [pkg]",                "Update packages"],
        ["search <query>",              "Search packages"],
        ["clean",                       "Clean caches"],
        ["connect [device]",            "Connect bluetooth device"],
        ["set env/toggle <v>",          "Modify env/toggles"],
        ["set mode <mode>",             "Change GPU mode"],
	["set autostart <command>",     "Set a package to autostart"]
        ["get env/toggle/log",          "View values/logs"],
        ["get autostart",               "List autostart entries"]
    ]

    for c in $cmds {
        print $"  (ansi green)($c.0)(ansi reset) .... (ansi purple)($c.1)(ansi reset)"
    }
    print ""
}

# ================= DISPATCH HELPERS =================

def dispatch-remove [args] {
    args_required $args 1
    match ($args | get 0) {
        "env"      => { args_required $args 2; remove-env ($args | get 1) }
        "toggle"   => { args_required $args 2; remove-toggle ($args | get 1) }
        "autostart" => { args_required $args 2; adel ($args | skip 1) }
        _          => { remove $args }
    }
}

def dispatch-set [args] {
    args_required $args 1
    match ($args | get -o 0) {
        "env"      => { args_required $args 3; set-env ($args | get 1) ($args | get 2) }
        "toggle"   => { args_required $args 3; set-toggle ($args | get 1) ($args | get 2) }
        "mode"     => { args_required $args 2; mode-set ($args | get 1) }
        "genuse"   => { args_required $args 4; genuse $args }
        "autostart" => { args_required $args 2; aset ($args | skip 1) }
        _          => { get_help }
    }
}

def dispatch-get [args] {
    args_required $args 1
    match ($args | get -o 0) {
        "env"       => get-env
        "toggle"    => get-toggle
        "log"       => build-log
        "genuse"    => { args_required $args 2; genuse $args }
        "autostart" => aget
        _           => get_help
    }
}

# ================= MAIN EXECUTION =================
export def --env --wrapped nudo [cmd: string, ...args: string] {
    if ($cmd =~ "-h") { help_command; return }

    match $cmd {
        "edit"     => { detect_os linux; args_required $args 1; edit $args }
        "install"  => { args_required $args 1; install $args }
        "remove"   => { dispatch-remove $args }
        "clean"    => { detect_os linux; clean }
        "update"   => { update $args }
        "search"   => { args_required $args 1; search ($args | get 0) }
        "connect"  => { detect_os linux; blueconnect ( if not ($args | is-empty) { $args | get 0 } ) }
        "set"      => { dispatch-set $args }
        "get"      => { dispatch-get $args }

        _ => {
            help_command
            error make {
                msg: "Invalid subcommand.",
                label: {
                    text: $"($cmd) is not a valid nudo command",
                    span: (metadata $cmd).span
                }
            }
        }
    }
}

