#!/usr/bin/env nu

use ($nu.default-config-dir | path join "functions/utils.nu") *


export def set-use [packagename: string, ...useflags: string] {
	##This is for set-use.
	any_one_of emerge ##Ensure only gentoo distro works.
	let priv = (any_one_of doas sudo run0)
	debug_print set-use: got package: $packagename and useflags: ($useflags | get 0 | str join ' ' | str trim) , writing to /etc/portage/package.use/($packagename | split row "/" | get 1)
	let write_file = $"/etc/portage/package.use/($packagename | split row "/" | get 1)"
	if not ($write_file | path exists) {"" | ^$priv tee $write_file}
	$"($packagename) ($useflags | get 0 | str join ' ' | str trim)" | ^$priv tee $write_file
}

export def get-use [] {
	any_one_of emerge ##Ensure only gentoo distro works.
	debug_print get-use: Getting useflags.
	let read_dir = $"/etc/portage/package.use"
	let files = (fd --type file --search-path $read_dir | lines)
	debug_print get-use: Files: ($files | to text)
	let useflags = $files | each {|file| open $file | parse "{Family}/{Package} {UseFlags}" } | to text | parse "Family: {Family}, Package: {Package}, UseFlags: {UseFlags}"
	$useflags
}

export def remove-use [packagename: string, ...useflags: string] {
	any_one_of emerge ##Ensure only gentoo distro works.
	debug_print remove-use: Removing the useflags: ($useflags | str join ',') from /etc/portage/package.use/($packagename | split row "/" | get 1)
	let read_file = $"/etc/portage/package.use/($packagename | split row '/' | get 1)"
	let priv = (any_one_of doas sudo run0)
	^$priv rm $read_file
}
