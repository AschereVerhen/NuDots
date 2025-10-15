#!/usr/bin/env nu


 export def edit [path_list: list<string>] {
	#preferred way	<--------	      Posix Way<---------
	#			|				|
	let editor_found = ([($env.config.buffer_editor), ($env.EDITOR)]
		| where {|it| not ($it | is-empty)}
		| first
	)
	if (echo $editor_found | is-empty) {
		print -e "No Default editor found. please set one of these variable in your env.nu file: " "$env.config.buffer_editor" "$env.EDITOR"
		exit 1
	}
	for path in $path_list {
		let file_create = not ($path | path exists)
		if not ($file_create) and not (($path | path type) == "file") {
			print -e "Disasterous! The path you entered is not a file! exitting."
			exit 1
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
		if ($og_contents == $new_contents) {
			exit 0
		}
		try {
			open $buffer_file | save --force $path
		} catch {
			sudo nu --commands $"open ($buffer_file) | save --force ($path)"
		}
	}

}
