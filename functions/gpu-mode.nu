#!/usr/bin/env nu

use ($nu.config-path | path dirname | path join "functions/utils.nu") dependency_check

export def mode-set [mode: string] {
	
	dependency_check "nvidia-smi"

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
			error make {
				msg: $"Option: ($mode) not found.",
				exit_code: 1
			}
		},
	}

	sudo nu --commands $"^nvidia-smi -pm 1; ^nvidia-smi ($NVIDIA_ARGS)" | ignore
}
