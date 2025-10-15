#!/usr/bin/env nu

##Define which functions nudo can access:
const functions_dir = ("~/.config/nushell/functions" | path expand)
use ($functions_dir | path join "editsu.nu") * ##Import editsu. MAKE SURE THIS ISNT IN MOD.NU!!!!
use ($functions_dir | path join "gpu-mode.nu") *
use ($functions_dir | path join "pacman_wrapper.nu") *
use ($functions_dir | path join "bluecon.nu") *
export def nudo [function: string, ...args: string] {

	match $function  {
		"edit" => {
			edit $args
		}, 
		"set-mode" => {
			mode-set ($args | get 0)
		},
		"install" => {
			install $args
		},
		"remove" => {
			remove $args
		},
		"clean" => {
			if not ($args | is-empty) {
				print -e "Warning: clean does not take any arguments. ignoring: " $args
			}
			clean
		},
		"update" => {
			update $args	
		}
		"connect" => {
			blueconnect ($args | get 0)
		}
		_ => {
			print -e "Function does not exists or is not imported."
		}
	}
};
