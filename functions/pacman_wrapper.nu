#!/usr/bin/env nu

def figure_out_pkg_manager [] {
	(["paru", "yay", "pacman", "emerge"]
		| where { |it| not (which $it | is-empty)}
		| first
	)
}
export def install [package: list<string>] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = match $pkg_manager {
		"pacman" | "emerge" => $"sudo ($pkg_manager)",
		_ => $pkg_manager
	}
	match $pkg_manager {
		"paru" | "yay" | "pacman" => { 
			^$command -S --noconfirm --color=always ...$package 
		},
		"emerge" => { 
			^$command -qv ...$package 
		}
	}
}

export def remove [package: list<string>] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = match $pkg_manager {
		"pacman" | "emerge" => $"sudo ($pkg_manager)",
		_ => $pkg_manager
	}
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			^$command -Rns --noconfirm ...$package
		},
		"emerge" => {
			^$command -C ...$package		
		}
	}
}

export def clean [] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = match $pkg_manager {
		"pacman" | "emerge" => $"sudo ($pkg_manager)",
		_ => $pkg_manager
	}
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {sudo rm -rf /var/cache/pacman/pkg
			sudo rm -rf ($env.HOME | path join .cache/paru)
			sudo rm -rf ($env.HOME | path join .cache/yay)
			sudo pacman -Scc --noconfirm | ignore
		},
		"emerge" => {
			sudo emerge --depclean
		}
	}
}

export def update [optional_packages: list<string> = [""]] {
	let pkg_manager = (figure_out_pkg_manager)
	let command = match $pkg_manager {
		"pacman" | "emerge" => $"sudo ($pkg_manager)",
		_ => $pkg_manager
	}
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			^$command -Syu --noconfirm ...$optional_packages
		},
		"emerge" => {
			^$command -qvuDN @world ...$optional_packages
		}
	}
}



export def search [SearchTerm: string] {
	let pkg_manager = (figure_out_pkg_manager) 
	let command = match $pkg_manager {
		"pacman" | "emerge" => {
			$"sudo ($pkg_manager)"
		},
		_ => {
			$pkg_manager
		}
	}
	match $pkg_manager {
		"paru" | "yay" | "pacman" => {
			^$command -Ss ...$SearchTerm
		},
		"emerge" => {
			^$command --search ...$SearchTerm
		}
	}
}

