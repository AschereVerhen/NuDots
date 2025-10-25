#!/usr/bin/env nu

use ($nu.config-path | path dirname | path join "functions/utils.nu") *

 export def edit [path_list: list<string>] {
	#preferred way	<--------	      Posix Way<---------
	#			|				|
	let editor_found = any_one_of ($env.config.buffer_editor?) ($env.EDITOR?) "nvim" "vim" "nano" "hx" "vi"
	for path in $path_list {
		let file_create = not ($path | path exists)
		if not ($file_create) and not (($path | path type) == "file") {
			error make {
				msg: $"(ansi red) Disasterous! ($path) Is a directory! (ansi reset)",
				exit_code: 1,
			}
		}

		##Now: The temporary directories.
		let temp_dir = ($"/tmp/nudo")

		if not ($temp_dir | path exists) {
			mkdir $temp_dir
		}
		let buffer_file = ($temp_dir | path join ($path | path basename))
		if not ($file_create) {
			open $path | save --force $buffer_file
		} else {
			echo "" | save --force $buffer_file
		}

		nu --commands $"^($editor_found) ($buffer_file)" ##Start Editting! Finally.

		let og_contents = if ($file_create) { "" } else { (open $path | to text) } 
		let new_contents = (open $buffer_file | to text)
		if not ($og_contents == $new_contents) {	
			try {
				open $buffer_file | save --force $path
			} catch {
				sudo nu --commands $"open ($buffer_file) | save --force ($path)"
			}
		}
	}

}
