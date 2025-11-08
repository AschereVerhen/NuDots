#!/usr/bin/env nu


use ($nu.config-path | path dirname | path join "functions" | path join utils.nu) *

def get_devices [] {
	mut counter = 1
	while (^bluetoothctl devices | is-empty) and ($counter != 10) {
		print -e $"Error: No devices found during searching phase. Waiting for 10 seconds and trying again\(Tries: ($counter)/10\)"
		sleep 10sec
		##Ensure once more
		run bluetoothctl power on
		run bluetoothctl scan on
		$counter += 1
	} ##The escape condition will only be true if ^bluetoothctl devices | is-empty returned false

	let list_of_devices = (bluetoothctl devices | split row "\n" | parse "Device {mac} {name}")

	return $list_of_devices
}

export def blueconnect [search_term: string = ""] {
	##Dependencies check
	dependency_check bluetoothctl fzf
	##Dependency check over

	##Ensuring bluetoothctl is up and running...
	run bluetoothctl power on
	run bluetoothctl scan on
	##Exit early if any of the commands failed.

	let list_of_devices = (get_devices)
	mut name_table = []
	if ($search_term | is-empty) {
		$name_table = ($list_of_devices | get name)
	} else {
		$name_table = ($list_of_devices | get name | find -i $search_term)
	}
	if (($name_table | length) > 1) {
		let user_select = ($name_table | str join "\n"
		| fzf --height 20 --prompt "Choose a device: ")
		if not ($user_select | is-empty) {
			let mac = ($list_of_devices | find $user_select | get mac | get 0)
			run bluetoothctl connect $mac
		}
	} else if (($name_table | length) == 1) {
		let mac = ($list_of_devices | find ($name_table | get 0) | get mac | get 0)
		run bluetoothctl connect $mac
	} else {
		print "unhandleded Error occured."
	}
}
