#!/usr/bin/env nu

export def wm [arg: string = "hyprland", ...optional_args: string] {
	if ($arg == "hyprland" and ($env.XDG_CURRENT_DESKTOP? | is-empty)) {
		"true" | save --force ($env.PERSISTENT_CONFIG | path join "colors")
		hyprland
	}
}
