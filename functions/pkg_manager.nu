#!/usr/bin/env nu

const utils_file = ($nu.default-config-dir | path join "functions/utils.nu")
use $utils_file *

def figure_out_pkg_manager [] {
	any_one_of paru yay pacman emerge winget
}

def priv_finder [] {
	any_one_of sudo doas run0 
}

export def install [package: list<string>] {
	let pkg_manager = (figure_out_pkg_manager)
	let priv = (priv_finder)
	match $pkg_manager {
		"paru" | "yay" => { 
			run $pkg_manager -S --noconfirm --color=always ...$package 
		},
		"pacman" => {
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
	let priv = (priv_finder)
	match $pkg_manager {		
		"paru" | "yay" => { 
			run $pkg_manager -Rns --noconfirm --color=always ...$package 
		},
		"pacman" => {
			run $priv $pkg_manager -Rns --noconfirm --color=always ...$package 
		},
		"emerge" => {
			run $priv $pkg_manager -C ...$package		
		},
		"winget" => { run $pkg_manager uninstall ...$package --source winget }
	}
}

export def clean [] {
	let pkg_manager = (figure_out_pkg_manager)
	let priv = (priv_finder)
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $priv rm -rf ($env.HOME | path join .cache/paru)
			run $priv rm -rf ($env.HOME | path join .cache/yay)
			run $priv pacman -Scc --noconfirm
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
	let priv = (priv_finder)
	match $pkg_manager {		
		"paru" | "yay" => { 
			run $pkg_manager -Syu --noconfirm --color=always ...$optional_packages 
		},
		"pacman" => {
			run $priv $pkg_manager -Syu --noconfirm --color=always ...$optional_packages 
		},
		"emerge" => {
			run $priv $pkg_manager -qvuDN @world ...$optional_packages
		},
		"winget" => { run $pkg_manager update --all --source winget --accept-package-agreements }
	}
}



export def search [search_term: string] {
	let pkg_manager = (figure_out_pkg_manager)
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $pkg_manager -Ss $search_term
		},
		"emerge" => {
			run $pkg_manager --search $search_term
		},
		"winget" => { run $pkg_manager search $search_term --source winget}
	}
}

export def list [] {
	let pkg_manager = (figure_out_pkg_manager)
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			run $pkg_manager -Q | parse "{Package} {Version}"
		},
		"emerge" => {
			dependency_check qlist
			run qlist -I | parse "{Family}/{Package}"
		}
	}
}
export def build-log [] {
	let pkg_manager = (figure_out_pkg_manager)
		
    	if $pkg_manager != "emerge" {
        	error make {
            		msg: "This function is only for Gentoo Linux."
            		label: {
                		text: $"Required: emerge, Found: ($pkg_manager)"
                		span: (metadata $pkg_manager).span
            		}
            		error_code: 1
        	}
    	}

    	let directories = (
        	fd --search-path /var/tmp/portage -d 2
        	| lines
        	| parse "/var/tmp/portage/{Family}/{Package}/"
        	| where { |row| $row.Package | is-not-empty }
    	)

    	if ($directories | length) == 0 {
        	return
    	} else if ($directories | length) == 1 {
        	let f = ($directories | get 0 | get Family)
        	let p = ($directories | get 0 | get Package)
        	sudo tail -f $"/var/tmp/portage/($f)/($p)/temp/build.log"
    	} else {
        	let new_table = (
            		$directories
            		| each {|dir| $"($dir.Family)/($dir.Package)" }
        	)
        	let selected_package = (
            		$new_table
			| to text
            		| fzf --prompt "Select a package"
        	)
        	sudo tail -f $"/var/tmp/portage/($selected_package)/temp/build.log"
    	}
}

