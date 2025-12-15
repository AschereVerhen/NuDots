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
		let selected_cmd = ($terminal_history | lines | reverse | uniq | to text | fzf)
		nu --commands $selected_cmd #run.
	}
}

export def fzf-directory-init --env [] {
	$env.config.keybindings ++= [{
		name: "Transverse the file structure with fzf.",
		modifier: control,
		keycode: char_t,
		mode: emacs,
		event: [
			{edit: CutFromLineStart},
			{edit: InsertString,
			value: "fzf-directory "},
			{edit: Paste},
			{send: Enter},
			{edit: Clear}
		]
	}]
}

export def fzf-directory --env [pwd: string = ""] {
	dependency_check fzf fd
	let $pwd = if ($pwd | is-not-empty) {
		$pwd
	} else {
		(pwd)
	}
	let directories = (fd -H --search-path $pwd)
	cd ($directories | fzf  --walker-skip=target,proc,sys,dev,.git | if (($in | path type) == "dir") {$in} else {($in | path dirname)})
}


export def fzf-edit-init --env [] {
	$env.config.keybindings ++= [{
		name: "Quickly Edit a file with fzf",
		modifier: control,
		keycode: char_e,
		mode: emacs,
		event: [
			{edit: CutFromLineStart },
			{edit: InsertString,
			value: "fzf-edit "},
			{edit: Paste},
			{send: Enter},
			{edit: Clear}
		]
	}]
}

export def fzf-edit [pwd: string = ""] {
	dependency_check fzf fd bat
	let $pwd = if ($pwd | is-not-empty) {
		$pwd
	} else {
		(pwd)
	}
	let directories = (fd -H --search-path $pwd)
	edit [($directories | fzf --preview "bat --color=always {}" --walker-skip=target,proc,sys,dev,.git )]
}

export def fzf-pictures-init --env [] {
	$env.config.keybindings ++= [{
		name: "Show a picture on terminal using kitten protocol",
		modifier: control,
		keycode: char_p,
		mode: emacs,
		event: [
			{edit: CutFromLineStart },
			{edit: InsertString, value: "fzf-pictures "},
			{edit: Paste},
			{send: Enter},
			{edit: Clear},
		]
	}]
}

##Write a picture to terminal Credit: jochumdev. <rene@jochum.dev>

export def fzf-pictures [pwd: string = ""] {
	dependency_check fzf fd bat kitten 
	let $pwd = if ($pwd | is-not-empty) {
		$pwd
	} else {
		(pwd)
	}
	##Terminal size:
	let size = (term size)

	let directories = (fd -H --search-path $pwd --type file)
	kitten i ($directories | find ".jpg" --no-highlight | to text | fzf --walker-skip=target,proc,sys,dev,.git --preview $'kitten icat --clear --transfer-mode=memory --place=$"($size | get columns)x($size | get rows)@0x0" --stdin=no {}')
}


export def init-all --env [] {
	fzf-history-init
	fzf-directory-init
	fzf-edit-init
	fzf-pictures-init
}


