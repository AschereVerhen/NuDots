#!/usr/bin/env nu

const utils_file = ($nu.default-config-dir | path join "functions/utils.nu")
use $utils_file *

def figure_out_pkg_manager [] {
	any_one_of paru yay pacman emerge winget
}

export def install [package: list<string>] {
	let pkg_manager = (figure_out_pkg_manager)
	let priv = any_one_of sudo doas run0
	match $pkg_manager {
		"paru" | "yay" | "pacman" => { 
			run $priv $pkg_manager -S --noconfirm --color=always ...$package 
		},
		"emerge" => { 
			run $priv $pkg_manager -qv ...$package
		},
		"winget" => { run $pkg_manager install ...$package --source winget --accept-package-agreements }
	}
}

export def remove [package: list<string>] {
	let pkg_manager = (figure_out_pkg_manager)
	let priv = any_one_of sudo doas run0
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $priv $pkg_manager -Rns --noconfirm ...$package
		},
		"emerge" => {
			run $priv $pkg_manager -C ...$package		
		},
		"winget" => { run $pkg_manager uninstall ...$package --source winget }
	}
}

export def clean [] {
	let pkg_manager = (figure_out_pkg_manager)
	let priv = any_one_of sudo doas run0
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
			run $priv $pkg_manager --depclean
		}
	}
}

export def update [optional_packages: list<string> = [""]] {
	let pkg_manager = (figure_out_pkg_manager)
	let priv = any_one_of sudo doas run0
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $priv $pkg_manager -Syu --noconfirm ...$optional_packages
		},
		"emerge" => {
			run $priv $pkg_manager -qvuDN @world ...$optional_packages
		},
		"winget" => { run $pkg_manager update --all --source winget --accept-package-agreements }
	}
}



export def search [search_term: string] {
	let pkg_manager = (figure_out_pkg_manager)
	let priv = any_one_of sudo doas run0
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $priv $pkg_manager -Ss $search_term
		},
		"emerge" => {
			run $priv $pkg_manager --search $search_term
		},
		"winget" => { run $pkg_manager search $search_term --source winget}
	}
}

