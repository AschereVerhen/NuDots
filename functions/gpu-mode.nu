#!/usr/bin/env nu


export def mode-set [mode: string] {
	if (which nvidia-smi | is-empty) {
		print "nvidia-smi Does not exit. Please install it."
		exit 1
	}

	let NVIDIA_ARGS = match ($mode | str downcase) {
		"powersave" | "p" => {
			"-lgc 100,100"
		},
		"balanced" | "b" => {
			"-lgc 500,800"
		},
		"max" | "gaming" | "g" => {
			"-lgc 1950,1950"
		},
		_ => {
			exit 0
		},
	}

	sudo nu --commands $"^nvidia-smi -pm 1; ^nvidia-smi ($NVIDIA_ARGS)" | ignore
}
