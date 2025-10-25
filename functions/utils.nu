#!/usr/bin/env nu


##This is the utils file for reuseable functions....


export def dependency_check [...programs: string] {
	let not_there = ($programs | where { |package| 
		(which $package | is-empty)
	})
	if not ($not_there | is-empty) {
		error make {
			msg: $"($not_there | str join ', ') Not found. Please install the packages.",
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
			msg: $"None of the programs: ($programs | str join ', ') were installed.",
			exit_code: 1
		}
	} else {
		return ($there | get 0)
	}
}


export def run [...command: string] {
	try {
		^$command
	} catch {
		error make {
			msg: $"Command: `($command | str join ' ')` failed.",
			exit_code: 1,
		}
	}
}
