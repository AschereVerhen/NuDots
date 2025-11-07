#!/usr/bin/env nu

##Define which functions nudo can access:
const functions_dir = ($nu.config-path | path dirname | path join "functions" | path expand)
use ($functions_dir | path join "editsu.nu") * ##Import editsu. MAKE SURE THIS ISNT IN MOD.NU!!!!
use ($functions_dir | path join "gpu-mode.nu") *
use ($functions_dir | path join "pkg_manager.nu") *
use ($functions_dir | path join "bluecon.nu") *
use ($functions_dir | path join "settings.nu") *

def help_command [] {
    print ""
    print $"(ansi blue)nudo - Administrative helper command for system tasks.(ansi reset)"
    print "---------------------------------------------------------"
    print $"(ansi red)Usage:(ansi reset) nudo <command> [arguments]"
    print ""
    print "Available Commands:"
    print $"  (ansi green)edit <path>(ansi reset) ............ (ansi purple)Edit configuration files \(requires elevated privileges\).(ansi reset)"
    print $"  (ansi green)set-mode <mode>(ansi reset) ........ (ansi purple)Set the system's GPU/performance mode.(ansi reset)"
    print $"  (ansi green)install <package>(ansi reset) ...... (ansi purple)Install packages via the package manager.(ansi reset)"
    print $"  (ansi green)remove <package>(ansi reset) ....... (ansi purple)Remove packages via the package manager.(ansi reset)"
    print $"  (ansi green)update [package](ansi reset) ....... (ansi purple)Update all or specified packages.(ansi reset)"
    print $"  (ansi green)search <query>(ansi reset) ......... (ansi purple)Search for available packages.(ansi reset)"
    print $"  (ansi green)clean(ansi reset) .................. (ansi purple)Clean package manager caches.(ansi reset)"
    print $"  (ansi green)connect [device](ansi reset) ....... (ansi purple)Connects to a specified Bluetooth device.(ansi reset)"
    print $"  (ansi green)set <env/toggle>(ansi reset) ...... (ansi purple)Be able to set a specific toggle or set a new environmental variable(ansi reset)"
    print $"  (ansi green)get <envs/toggles>(ansi reset) ....... (ansi purple)Be able to get the current value of all settings or environmental variables declared..(ansi reset)"
    print ""
}

def detect_os [desired: string = ""] {
	##First check if the system is running any type of linux system.
	mut os = "linux"
	if ("/etc" | path exists) {
		if not ("linux" in (open /etc/os-release) or "Linux" in (open /etc/os-release)) {
			$os = "unix"
		}
	} else {
	$os = "windows"
	}

	if not (($desired | str downcase) in $os) {
		error make {
			msg: $"(ansi red) Required Os was ($desired) but found ($os)",
			error_code: 1
		}
	}
}

export def --env --wrapped nudo [function: string, ...args: string] {

	if ($function =~ "-h") { #This covers also -*-h*elp, and -h!
		help_command
		return
	}

	match $function  {
		"edit" => {
			detect_os linux
			edit $args
		}, 
		"set-mode" => {
			detect_os linux
			mode-set ($args | get 0)
		},
		"install" => {
			install $args
		},
		"remove" => {
			remove $args
		},
		"clean" => {
			detect_os linux
			clean
		},
		"update" => {
			update $args
		},
		"search" => {
			search ($args | get 0)
		}
		"connect" => {
			detect_os linux
			blueconnect (if not ($args | is-empty) { $args | get 0 })
		},
		"set" => {
			match ($args | get -o 0) {
				"env" => { set-env ($args | get 1) ($args | get 2) },
				"toggle" => { set-toggle ($args | get 1) ($args | get 2) },
				_ => get_help
			}
		},
		"get" => {
			match ($args | get -o 0) {
				"env" => { get-env },
				"toggle" => { get-toggle},
				_ => get_help
			}
		},


		_ => {
			print -e "Function does not exists or is not imported."
		}
	}
};

