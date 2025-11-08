#!/usr/bin/env nu


##This is the utils file for reuseable functions....


export def dependency_check [...programs: string] {
	let not_there = ($programs | where { |package| 
		(which $package | is-empty)
	})
	if not ($not_there | is-empty) {
		error make {
			msg: $"(ansi red)($not_there | str join ', ') Not found. Please install the packages.",
			exit_code: 1
		}
	} else {
		return 0
	}
}

export def any_one_of [...programs: string] {
	let there = ($programs | where {
		|package|
		(which $package | is-not-empty)
	});
	if ($there | is-empty) {
		error make {
			msg: $"(ansi red)None of the programs: ($programs | str join ', ') were installed.",
			exit_code: 1
		}
	} else {
		return ($there | get 0)
	}
}


export def run --wrapped [...command: string] {
	try {
		^$command
	} catch {|e|
		print $e
		let error_msg = $e.msg
		let error_span = $e | get json | from json | get labels | get span | get 0
		print $error_span
		error make {
			msg: $error_msg
			label: {
				text: $"(ansi red)Command: `($command | str join ' ')` failed.",
				span: $error_span
			}
			exit_code: $e.exit_code,
		}
	}
}
