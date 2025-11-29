#!/usr/bin/env nu


use ($nu.default-config-dir | path join "functions/utils.nu") *

def get_devices [] {
	mut counter = 1
	while (^bluetoothctl devices | is-empty) and ($counter != 10) {
		debug_print $"bluecon: ---Iteration: ($counter)---"
		print -e $"(ansi red)Error: No devices found during searching phase. Waiting for 10 seconds and trying again\(Tries: ($counter)/10\)"
		sleep 10sec
		##Ensure once more
		run bluetoothctl power on
		run bluetoothctl scan on
		$counter += 1
	} ##The escape condition will only be true if ^bluetoothctl devices | is-empty returned false
	if ($counter < 10) {
		let list_of_devices = (bluetoothctl devices | split row "\n" | parse "Device {mac} {name}")
		debug_print $"bluecon.nu: $list_of_devices: ($list_of_devices)"
		return $list_of_devices
	}
	error make {
		msg: "Bluetooth scan time out.",
		label: {
			text: "No Devices were found.",
		},
		error_code: 1
	}
}

export def blueconnect [search_term: string = ""] {
	##Dependencies check
	dependency_check bluetoothctl fzf
	##Dependency check over

	let list_of_devices = (get_devices)
	mut name_table = []
	if ($search_term | is-empty) {
		$name_table = ($list_of_devices | get name)
		debug_print $"bluecon.nu: Went into the is-empty branch. $name_table: ($name_table)"
	} else {
		$name_table = ($list_of_devices | get name | find -i $search_term)
		debug_print $"bluecon.nu: Went into the is-not-empty branch. $name_table: ($name_table)"
	}
	debug_print $"bluecon: $name_table length: ($name_table | length)"
	if (($name_table | length) > 1) {
		let user_select = ($name_table | str join "\n"
		| fzf --height 20 --prompt "Choose a device: ")
		if not ($user_select | is-empty) {
			let mac = ($list_of_devices | find $user_select | get mac | get 0)
			debug_print $"bluecon: mac_address: ($mac)"
			run bluetoothctl connect $mac
		}
	} else if (($name_table | length) == 1) {
		let mac = ($list_of_devices | find ($name_table | get 0) | get mac | get 0)
		debug_print $"bluecon: mac_address: ($mac)"
		run bluetoothctl connect $mac
	} else {
		error make {
			msg: $"(ansi red)Unknown Error Occured.",
			label: {
				text: $"Here, Name Table is empty. Maybe there are no devices near you?",
				span: (metadata $name_table).span
			},
			exit_code: 255
		}
	}
}
