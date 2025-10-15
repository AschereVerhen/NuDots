#!/usr/bin/env nu

export def blueconnect [ssid: string = ""] {
	if (which bluetoothctl | is-empty) {
		print -e "Please install bluez-utils... this script depends upon it(for now)."
		exit 1
	}

	^bluetoothctl scan on

	let devices = (bluetoothctl devices | split row "\n" | split column --number 3 " " | get column2)
	let devices_prettyprint =  (bluetoothctl devices | split row "\n" | split column --number 3 " " | get column3)
	let fzf_input = ($devices_prettyprint | each { |it| echo $it } | str join "\n" | fzf)

	let device_length = ($devices | length)
	mut found = 0
	for i in 0..($device_length - 1) {
		let index = $i
		if (($devices_prettyprint | get $i) == $fzf_input) {
			 $found += $i
		} else { continue }
	}
	let device_mac = ($devices | get $found)

	^bluetoothctl connect $device_mac
}

