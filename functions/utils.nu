#!/usr/bin/env nu


##This is the utils file for reuseable functions....


export def dependency_check [...programs: string] {
	let not_there = ($programs | each {
		|it|
		if (which $it | is-empty) {
			echo $it
		}
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
