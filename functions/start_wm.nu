#!/usr/bin/env nu
const settings_file = ($nu.default-config-dir | path join "functions/settings.nu")
use $settings_file get-toggle
use ($nu.default-config-dir | path join "functions/utils.nu") [detect_os]

def start_normal_wm_or_startx [wm: any] {
	if ($env.XDG_CURRENT_DESKTOP? | is-not-empty) {
		return #do not go further
	}
	let $wm = if ($wm | is-not-empty) {
		($wm | into string)
	} else {
		return #do not execure anymore.
	}
	if not ($nu.data-dir | path exists) { mkdir $nu.data-dir }
	^$wm | save -f ($nu.data-dir | path join "WindowManager.log")
}


export def wm [] {
	detect_os linux bsd
	let window_manager = (get-toggle | find wm | get value? | get 0?)
	start_normal_wm_or_startx $window_manager ##wms like startx, hyprland, i3 should fall into this branch and further code should not be executed...
}
