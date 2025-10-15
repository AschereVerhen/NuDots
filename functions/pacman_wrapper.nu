#!/usr/bin/env nu

def figure_out_aur_helper [] {
	(["paru", "yay", "pacman"]
		| where { |it| not (which $it | is-empty)}
		| first
	)
}
export def install [package: list<string>] {
	let aur_package = (figure_out_aur_helper)
	^$aur_package -S --noconfirm --color=always ...$package 
}

export def remove [package: list<string>] {
	let aur_package = (figure_out_aur_helper)
	^$aur_package -Rns --noconfirm --color=always ...$package	
}

export def clean [] {
	sudo rm -rf /var/cache/pacman/pkg
	rm -rf ($env.HOME | path join .cache/paru/clone)
	rm -rf ($env.HOME | path join .cache/paru/diff)
	sudo pacman -Scc --noconfirm | ignore
}

export def update [optional_packages: list<string> = [""]] {
	let aur_helper = (figure_out_aur_helper)
	^$aur_helper -Syu --noconfirm ...$optional_packages
}
