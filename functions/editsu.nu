#!/usr/bin/env nu

use ($nu.config-path | path dirname | path join "functions/utils.nu") *

export def edit [path_list: list<string>] {
	let editor_found = any_one_of ($env.config.buffer_editor?) ($env.EDITOR?) "nvim" "vim" "nano" "hx" "vi"
	debug $"editsu.nu: Found editor: ($editor_found)"
	for path in $path_list {
		debug $"editsu: Current path: ($path)"
		let file_create = not ($path | path exists)
		if not ($file_create) and (($path | path type) == "dir") {
			error make {
				msg: $"(ansi red)Disasterous! ($path) Is a directory! (ansi reset)",
				exit_code: 1,
			}
		}

		##Now: The temporary directories.
		let temp_dir = ($"/tmp/nudo")

		if not ($temp_dir | path exists) {
			mkdir $temp_dir
			debug $"Creating directory: ($temp_dir)"
		}
		let buffer_file = ($temp_dir | path join ($path | path basename))
		if not ($file_create) {
			debug $"editsu: ($path) already exists. Not creating the file."
			open $path | save --force $buffer_file
		} else {
			echo "" | save --force $buffer_file #USE ECHO INSTEAD OF PRINT.
			#print will convert "" => null, save will malfunction. Disaster.
			#Echo on the other hand will not.
			debug $"editsu: ($path) does not exists. Creating a file."
		}

		##Implimenting Atomic Lock System.
		let lock_database = "/tmp/nudo/locks";
		if not ($lock_database | path exists) {mkdir $lock_database; debug $"Creating Directory: ($lock_database)"};
		##Check if a lock for this particular file.
		let lock_file = ($lock_database | path join $"($path | hash sha256).lock");
		debug $"Lockfile: ($lock_file)"
		if ($lock_file | path exists) {
			error make {
				msg: $"(ansi red)It seems that another instance of nushell is already editing this file. Will not continue further. Please remove (ansi green)($lock_file)(ansi red) If you want to edit this file anyways.(ansi reset)",
				label: {
					text: $"(ansi red)Please remove (ansi green)($lock_file)(ansi red) If you want to edit this file anyways.(ansi reset)",
					span: (metadata $lock_file).span,
				},
				exit_code: 1
			}
		} else {
			debug $"Locking file."
			touch $lock_file
		}


		nu --commands $"^($editor_found) ($buffer_file)" ##Start Editting! Finally.

		let og_contents = if ($file_create) { "" } else { (open $path | to text) } 
		let new_contents = (open $buffer_file | to text)
		debug $"Write file? ($og_contents != $new_contents)"
		if not ($og_contents == $new_contents) {	
			try {
				open $buffer_file | save --force $path
				debug "Writing file without priviledge escalation."
			} catch {
				^(any_one_of sudo doas run0) nu --commands $"open ($buffer_file) | save --force ($path)"
				debug $"Priviledge escalted using (any_one_of sudo doas run0)"
			}
		}
		rm $lock_file
		debug "Removed lockfile."
	}
}
