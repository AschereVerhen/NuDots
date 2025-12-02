#!/usr/bin/env nu

export def get_help [] {
	print "------------------------------------------------------------------------------------------------------------"
	print $"(ansi green)This is the options you can fine tweak/add environmental variables(ansi reset)"
	print $"(ansi white)Usage:(ansi reset) nudo (ansi yellow)set/get (ansi reset)(ansi purple)<dispatcher>(ansi reset)"
	print $"(ansi yellow)dispatchers: " #Continue THIS!
	print $"(ansi purple)set env(ansi reset) ............... (ansi cyan)Set a new environmental variable."
	print $"(ansi purple)get env(ansi reset) ............... (ansi cyan)Get a list of all environmental variables."
	print $"(ansi purple)set toggle(ansi reset) ............ (ansi cyan)Set a toggle."
	print $"(ansi purple)get toggle(ansi reset) ............ (ansi cyan)Get all toggles."
	print $"(ansi purple)set mode(ansi reset) .............. (ansi cyan)Set performance profiles on an nvidia gpu."
	print "------------------------------------------------------------------------------------------------------------"
	print $"(ansi yellow)List of toggles: (ansi reset)"
	print $"(ansi purple)1. Color .......................... (ansi cyan)Toggle whether pywal should be executed on .current_image in ($nu.home-path)/Pictures\(this is to be symlinked to the current wallpaper!\) or the wallpath you set."
	print $"(ansi purple)2. wm ............................. (ansi cyan)Select Which window manager should start. \(Note: For x11 wms, just put here startx.\)"
	print $"(ansi purple)3. wallpath ....................... (ansi cyan)Select a wallpaper path which will be used to generate pywal colors."
	print $"(ansi purple)4. DEBUG .......................... (ansi cyan)Turn on debugging mode for more debug print statements. \(Set value to 1\)"
	print $"(ansi purple)5. powersave [min,max] ............ (ansi cyan)Set Memclocks for powersaving mode for nvidia gpus."
	print $"(ansi purple)6. balanced [min,max] ............. (ansi cyan)Set Memclocks for balanced mode."
	print $"(ansi purple)7. gaming [min,max] ............... (ansi cyan)Set Memclocks for gaming/max mode."
} 

##Fetchs the custom envs set 
export def get-env [] {
	let env_file = ($nu.default-config-dir |  path join "env.nu")
	let envs = (open ($env_file) | parse "$env.{name} = \"{value}\"")
	$envs
}

def write-env --env [env_list: list<any>] {
	const env_file = ($nu.default-config-dir | path join "env.nu")
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

export def remove-env [env_name: string] {
	write-env (get-env | where {|table| $table.name != $"($env_name)"})
}

export def get-toggle [] {
	let save_file = ($nu.data-dir | path join "toggles")
	if not ($save_file | path exists) { "" | save -f $save_file}
	open $save_file | from json  
}

def write-toggle [toggles: list<any>] {
	let save_file = ($nu.data-dir | path join "toggles")
	"" | save -f $save_file
	$toggles | to json | save -f $save_file
}

export def set-toggle [toggle: string, value: string] {
	let save_file = ($nu.data-dir | path join "toggles")
	if (open $save_file | is-empty) or not ($save_file | path exists) {"" | save -f $save_file}
	let toggle_table = (open $save_file | from json)
	debug_print set-toggle: $toggle_table
	##$save_file is now in json
	debug_print set-toggle: Is toggle_table empty? ($toggle_table | is-empty | to text)
	debug_print set-toggle: Is ($toggle) in toggle_table ? ($toggle in ($toggle_table | table) | to text)
	
	if ($toggle_table | is-empty) {
		debug_print "set-toggle: toggle_table is empty."
		let table_to_append = [[toggle, value]; [$toggle, $value]];
		write-toggle ($toggle_table | append $table_to_append)
		return
	}

	if ($toggle in ($toggle_table | table)) {
		debug_print set-toggle: desired branch
		let new_table = (
			get-toggle | each {|row|
				debug_print set-toggle: current toggle: $row.toggle . Desired toggle: $toggle
				if $row.toggle == $toggle {
					debug_print set-toggle: new value: $value
					$row | upsert value $value
				} else {
					debug_print set-toggle: not the desired toggle, will not change value.
					$row
				}
			}
		)
		write-toggle $new_table
	} else {
		debug_print set-toggle: Else branch.
		let table_to_append = [[toggle, value]; [$toggle, $value]];
		write-toggle ($toggle_table | append $table_to_append)
	}
}
export def remove-toggle [toggle: string] {
	write-toggle (get-toggle | where {|table| $table.toggle != $"($toggle)"})
}
