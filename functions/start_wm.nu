#!/usr/bin/env nu
const settings_file = ($nu.default-config-dir | path join "functions/settings.nu")
use $settings_file set-toggle

export def wm [arg: string = "hyprland", ...optional_args: string] {
	if ($arg == "hyprland" and ($env.XDG_CURRENT_DESKTOP? | is-empty)) {
		set-toggle colors "true"
		hyprland
	}
}
