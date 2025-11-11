#!/usr/bin/env nu
const settings_file = ($nu.default-config-dir | path join "functions/settings.nu")
use $settings_file get-toggle
use ($nu.default-config-dir | path join "functions/utils.nu") detect_os

export def wm [] {
	detect_os linux bsd
	if ($env.XDG_CURRENT_DESKTOP? | is-empty) {
		let window_manager = (get-toggle | find wm | get value? | get 0?)
		if not ($nu.data-dir | path exists) { mkdir $nu.data-dir }
		if ($window_manager | is-not-empty) {
			^$window_manager | save -f ($nu.data-dir | path join "WindowManager.log")
		}
	}
}
