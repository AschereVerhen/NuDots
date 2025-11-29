#!/usr/bin/env nu


##This is the utils file for reuseable functions....


export def dependency_check [...programs: string] {
	let not_there = ($programs | where { |package| 
		(which $package | is-empty)
	})
	if not ($not_there | is-empty) {
		let span = (metadata $programs).span
		error make {
			msg: $"(ansi red)($not_there | str join ', ') Not found. Please install it.",
			label: {
				text: "Dependency Check failed.",
				span: $span
			}
			exit_code: 1
		}
	} else {
		return 0
	}
}

export def any_one_of [...programs: string] {
	let there = ($programs | where {
		|package|
		if ($package | is-not-empty) {(which $package | is-not-empty)} else {null}
	});
	if ($there | is-empty) {
		let span = (metadata $programs).span
		error make {
			msg: $"(ansi red)None of the programs: ($programs | str join ', ') were installed.",
			label: {
				text: "Atleast One of the above was required. None were installed.",
				span: $span
			}
			exit_code: 1
		}
	} else {
		return ($there | get 0)
	}
}


export def run --wrapped [...command: string] {
	let span = (metadata $command).span
	try {
		^$command	
	} catch {|e|
		let error_msg = $e.msg
		error make {
			msg: $error_msg
			label: {
				text: $"(ansi red)Command: `($command | str join ' ')` failed.",
				span: $span
			}
			exit_code: 1,
		}
	}
}
export def detect_os [...allowed: string] {
	##First check if the system is running any type of linux system.
	let os = ($nu.os-info.name)
	if not ($os in $allowed) {
		let span = (metadata $allowed).span
		let prettified = if (($allowed | length) == 1) {$allowed | get 0} else {$allowed | str join ", or "}
		error make {
			msg: $"(ansi red)Invalid Platform. This function is not available for your platform."
			label: {
				text: $"Required: ($prettified), found: ($os)",
				span: $span,
			},
			exit_code: 1,
		}
	}
}

export def args_required [args_list: list<string>, args_atleast: int] {
	let total_args = ($args_list | length)
	if ($total_args < $args_atleast) {
		let span = (metadata $total_args).span
		error make {
			msg: "Not Enough arguments supplied.",
			label: {
				text: $"Required args: ($args_atleast), got: ($total_args)",
				span: $span
			},
			exit_code: 1
		}
	}
}

export def debug [...statement: string] {
	##we print a debug statement iff DEBUG toggle is set to 1
	#implimenting a short get_toggle function to ensure utils.nu has no dependencies.
	let save_file = ($nu.data-dir | path join "toggles");
	if not ($save_file | path exists) { "" | save -f $save_file }
	if ((open $save_file | find "DEBUG") | is-empty) {
		return
	}
	print $"[DEBUG]: ($statement | str join ' ')"
}
