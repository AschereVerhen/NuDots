#!/usr/bin/env nu

export def get_help [] {
	print $"(ansi green)This is the options you can fine tweak/add environmental variables(ansi reset)"
	print $"(ansi white)Usage:(ansi reset) nudo (ansi yellow)set/get (ansi reset)(ansi purple)<dispatcher>(ansi reset)"
	print $"(ansi yellow)dispatchers: " #Continue THIS!
	print $"(ansi purple)set env(ansi reset).................(ansi cyan) Set a new environmental variable."
	print $"(ansi purple)get env(ansi reset).................(ansi cyan) Get a list of all environmental variables."
	print $"(ansi purple)set toggle(ansi reset).................(ansi cyan) Set a toggle."
	print $"(ansi purple)get toggles(ansi reset).................(ansi cyan) Get all toggles."
} 

##Fetchs the custom envs set 
export def get-env [] {
	let env_file = ($nu.config-path | path dirname | path join "env.nu")
	let envs = (open ($env_file) | parse "$env.{name} = \"{value}\"")
	$envs
}

def write-env --env [env_list: list<any>] {
	const env_file = ($nu.config-path | path dirname | path join "env.nu")
	"" | save -f $env_file ##Clear the file first.
	$env_list | each {|var|
		$"$env.($var.name) = \"($var.value)\"\n" | save --append $env_file
	}
	source $env_file
}

export def set-env --env [env_name: string, value: any, --help: string] {
	let name = ($env_name | to text)
	if not ($name in (get-env | get name)) {
		let table_a = get-env
		let table_b = [[name, value]; [$env_name, $value]]
		write-env ($table_a | append $table_b)
	} else {
		let new_table = (
			get-env | each {|row| 
				if $row.name == $env_name {
					$row | upsert value $value
				} else {
					$row
				}
			}
		)
		write-env $new_table
	}
}

export def get-toggle [] {
	let save_file = ($env.PERSISTENT_CONFIG | path join "toggles")
	if not ($save_file | path exists) { "" | save -f $save_file}
	open $save_file | parse "{toggle}: {value}" 
}

def write-toggle [toggles: list<any>] {
	let save_file = ($env.PERSISTENT_CONFIG | path join "toggles")
	"" | save -f $save_file
	$toggles | each {|vars| 
		$"($vars.toggle): ($vars.value)\n" | save --append $save_file
	} | to text
}

export def set-toggle [toggle: string, value: string] {
	let save_file = ($env.PERSISTENT_CONFIG | path join "toggles")
	
	if ($toggle in (get-toggle | get toggle)) {
		let new_table = (
			get-toggle | each {|row| 
				if $row.toggle == $toggle {
					$row | upsert value $value
				} else {
					$row
				}
			}
		)
		write-toggle $new_table
	} else {
		let table_c = get-toggle
		let table_d = [[toggle, value]; [$toggle, $value]];
		write-toggle ($table_c | append $table_d)
	}
}
