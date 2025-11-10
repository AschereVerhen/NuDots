#!/usr/bin/env nu

const utils_file = ($nu.default-config-dir | path join "functions/utils.nu")
use $utils_file *

def figure_out_pkg_manager [] {
	any_one_of paru yay pacman emerge winget
}

def make_command [pkg_manager: string] {
	let priv_cmd = any_one_of sudo doas run0
	let command = match $pkg_manager {
		"pacman" | "emerge" => {
			$"($priv_cmd) ($pkg_manager)"
		},
		_ => {
			$pkg_manager
		}
	}
	return $command
}

export def install [package: list<string>] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = make_command $pkg_manager
	match $pkg_manager {
		"paru" | "yay" | "pacman" => { 
			run $command -S --noconfirm --color=always ...$package 
		},
		"emerge" => { 
			run $command -qv ...$package 
		},
		"winget" => { run $pkg_manager install ...$package --source winget --accept-package-agreements }
	}
}

export def remove [package: list<string>] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = make_command $pkg_manager
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $command -Rns --noconfirm ...$package
		},
		"emerge" => {
			run $command -C ...$package		
		},
		"winget" => { run $pkg_manager uninstall ...$package --source winget }
	}
}

export def clean [] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = make_command $pkg_manager
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run sudo rm -rf ($env.HOME | path join .cache/paru)
			run sudo rm -rf ($env.HOME | path join .cache/yay)
			run sudo pacman -Scc --noconfirm
			##Removing orphaned package
			job spawn { pacman -Qdtq | parse "{name}" | each {|it| ^$pkg_manager -Rns --noconfirm $it.name} }
			print $"(ansi green)Removing Orphaned packages in the background..."
		},
		"emerge" => {
			run $command --depclean
		}
	}
}

export def update [optional_packages: list<string> = [""]] {
	let pkg_manager = (figure_out_pkg_manager)
	
	let command = make_command $pkg_manager
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $command -Syu --noconfirm ...$optional_packages
		},
		"emerge" => {
			run $command -qvuDN @world ...$optional_packages
		},
		"winget" => { run $pkg_manager update --all --source winget --accept-package-agreements }
	}
}



export def search [search_term: string] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = make_command $pkg_manager
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $command -Ss $search_term
		},
		"emerge" => {
			run $command --search $search_term
		},
		"winget" => { run $pkg_manager search $search_term --source winget}
	}
}

