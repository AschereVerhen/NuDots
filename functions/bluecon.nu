#!/usr/bin/env nu

def run [...command: string] {
	^$command | ignore ##run command
	if ($env.LAST_EXIT_CODE != 0) {
		error make {
			msg: $"Command Failed: $(command).",
			exit_code: 1
		}
	}
}


export def blueconnect [search_term: string = ""] {
	##Dependencies check
	##list of dependencies -> check if each "it" is available on the system -> ignore the value.
	["bluetoothctl", "fzf"] | each { |it|
		if (which $it | is-empty) {
			error make {
				msg: $"($it) not found. Please install the package.",
				exit_code: 1
			}
		}
	}
	##Dependency check over

	##Ensuring bluetoothctl is up and running...
	run bluetoothctl power on
	run bluetoothctl scan on
	##Exit early if any of the commands failed.
	let list_of_devices = (bluetoothctl devices | split row "\n" | split column -n 3 " " useless ssid name | reject useless) ##Reject the column with Device. 
	let name_table = ($list_of_devices | get name | find ($search_term))
	if (($name_table | length) > 1) {
		let user_select = ($name_table | each {
			|it| echo $it
		} | str join "\n"
		| fzf --height 20 --prompt "Choose a device: ")
		let ssid = ($list_of_devices | find $user_select | get ssid | get 0)
		run bluetoothctl connect $ssid
	} else {
		let ssid = ($list_of_devices | find $search_term | get ssid | get 0)
		run bluetoothctl connect $ssid
	}
}
