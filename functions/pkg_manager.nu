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
			run $priv rm -rf ($env.HOME | path join (".cache" | path join $pkg_manager))
			run $priv rm -rf /var/cache/pacman/pkg/*
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
	dependency_check fzf fd
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

export def help_use [] {
	    print ""
    print $"(ansi blue)nudo use - one stop management of your useflags. (ansi reset)"
    print "---------------------------------------------------------"
    print $"(ansi red)Usage:(ansi reset) nudo use set/get <package_name> flags/keywords/env-file \(only for gentoo.\)"
    print ""
    print "Available Commands:"
    print $"  (ansi green)use <package_name> useflags(ansi reset) ............ (ansi purple)Edit/set useflags. Lines wrote to /etc/portage/package.use/<package_base_name>.(ansi reset)"
    print $"  (ansi green)keyword(ansi reset) ..... (ansi purple)Quickly set a keyword.(ansi reset)"
    print $"  (ansi green)env(ansi reset) ...... (ansi purple)Quickly set a env file.(ansi reset)"
    print ""

}

def set_thing [thing: any, package: any, ...flags: string] {
	if (($thing | is-empty) or ($package | is-empty) or ($flags | is-empty)) {
		help_use
	}
	
	let thing = $thing | into string
	let package = $package | into string
	let package_base_name = ($package | parse "{family}/{package}" | get package | get 0)
	let formatted_string = $"($package) ($flags | str join ' ')"
	
	let export_file = (
		match $thing {
			"use" => {$"/etc/portage/package.use/($package_base_name)"},
			"keyword" => { $"/etc/portage/package.accept_keywords/($package_base_name)"},
			"env" => { $"/etc/portage/package.env/($package_base_name)"}
		}
	);

	##IF we already are root, we do not need sudo/doas/run0 whatever.
	try {
		$formatted_string | save --append --force $export_file
	} catch {
		$formatted_string | ^(any_one_of doas sudo run0) tee -a  $export_file
	}
}

def get_thing --env [thing: any] {
	if ($thing | is-empty) {
		help-use
	}
	let $thing = ($thing | into string)

	let read_dir = (
		match $thing {
			"use" => {"/etc/portage/package.use"},
			"keyword" => { "/etc/portage/package.accept_keywords"},
			"env" => { "/etc/portage/package.env"}
		}
	);
	#Now we will read all the files in read_dir and open em
	let files = (
		fd --base-directory $read_dir
		| to text
		| lines
	)
	cd $read_dir #just in case. I dont wanna code an implimentation to prepend /etc/portage/blah to the files list.
	let content = ($files | each {|file| open $file} | to text)
	let final_table = ($content | lines | where {|row| $row | is-not-empty} | parse "{family}/{package} {flags}")
	$final_table
}

export def genuse [$args: list<string>] {
	##If $thing == use, write to /etc/portage/package.use/<package_name>
	##If $thing == keyword, write to /etc/portage/package.accept_keywords/<package_name>
	##If $thing == env, write to /etc/portage/package.env/<package_name>
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
	let set_or_get = ($args | get -o 0)

	match $set_or_get {
		"set" => {
			if ($args | length) < 3 {
				help_use
				error make { msg: "The 'set' command requires a type (use/keyword/env) and a package." }
				return
			}

			let thing = ($args | get 1)
			let package = ($args | get 2)
			let flags = ($args | slice 3..) # This is safe, returns [] if no flags

			print $"(ansi green)Setting(ansi reset) ($thing) for ($package) with flags: ($flags)"
			set_thing $thing $package ...$flags
		},

		"get" => {
			if ($args | length) < 2 {
				help_use
				error make { msg: "The 'get' command requires a type (use/keyword/env)." }
				return
			}

			let thing = ($args | get 1)

			print $"(ansi green)Getting(ansi reset) ($thing)..."
			get_thing $thing
		},

		_ => {
			help_use
		}
	}
}
