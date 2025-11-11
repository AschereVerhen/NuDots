#!/usr/bin/env nu

use ($nu.default-config-dir | path join "functions/utils.nu") *
use ($nu.default-config-dir | path join "functions/editsu.nu") *
#Example Keybinding(just for reference):
#{
# 	name: "Insert fzf-history",
# 	modifier: Control,
# 	code: char_h,
#	mode: vi_insert,
# 	event: {edit: InsertString, 
# 		value: "Hello World"
# 	}
# }
##Firstly, lets do the most easiest function out of all? History one.

export def fzf-history-init --env [] {
	$env.config.keybindings ++= [{
		name: "Insert The selected command.", #Set the name
		modifier: control, #set the modifier. in this case ctrl
		keycode: char_h, #character. here it is h. 
		mode: emacs, #mandatory.
		event: [
			{edit: Clear} #clear the screen
			{edit: InsertString, #insert text to cursor
			value: "fzf-history"} #execute the function defined below. NOTE BOTH FUNCS MUST BE SOURCED
			{send: Enter} #enter. Comment this line and the --preview behaviour is done.
		]
	}]
}

export def  fzf-history [] {
	
	dependency_check fzf

	if not ($nu.history-path | path exists) {
		error make {
			msg: "history.txt does not exist. Are you in nushell?",
			label: {
				text: "File not found.",
				span: (metadata $nu.history-path).span
			},
			exit_code: 255
		}
	}
	let terminal_history = ($nu.history-path | open)
	if ($terminal_history | is-not-empty) {
		let selected_cmd = ($terminal_history | lines | reverse | to text | fzf)
		^$selected_cmd #run.
	}
}

export def fzf-file-init --env [] {
	$env.config.keybindings ++= [{
		name: "Transverse the file structure with fzf.",
		modifier: control,
		keycode: char_t,
		mode: emacs,
		event: [
			{edit: Clear},
			{edit: InsertString,
			value: "fzf-file"}
			{send: Enter},
			{edit: Clear}
		]
	}]
}

export def fzf-file --env [] {
	dependency_check fzf fd
	# let curr_dir = (pwd)
	let directories = (fd -H)
	cd ($directories | fzf | if (($in | path type) == "dir") {$in} else {($in | path dirname)})
}


export def fzf-edit-init --env [] {
	$env.config.keybindings ++= [{
		name: "Quickly Edit a file with fzf",
		modifier: control,
		keycode: char_e,
		mode: emacs,
		event: [
			{edit: Clear},
			{edit: InsertString,
			value: "fzf-edit"}
			{send: Enter},
			{edit: Clear}
		]
	}]
}

export def fzf-edit [] {
	dependency_check fzf fd bat
	let directories = (fd -H)
	edit [($directories | fzf --preview "bat --color=always {}")]
}



export def init-all --env [] {
	fzf-history-init
	fzf-file-init
	fzf-edit-init
}


