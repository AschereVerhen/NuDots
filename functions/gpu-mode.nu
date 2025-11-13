#!/usr/bin/env nu

use ($nu.config-path | path dirname | path join "functions/utils.nu") [dependency_check, detect_os, any_one_of]


def get_help [] {
	print "------------------------------------------------------------------------------------------------------------"
	print $"(ansi green)Options for gpu-mode(ansi reset)"
	print $"(ansi white)Usage:(ansi reset) nudo (ansi yellow)set (ansi reset)(ansi purple)<dispatcher>(ansi reset)"
	print $"(ansi yellow)dispatchers: " #Continue THIS!
	print $"(ansi purple)powersave | p(ansi reset) .............. (ansi cyan)Power saving profile."
	print $"(ansi purple)balanced | b(ansi reset) ............... (ansi cyan)Balanced profile"
	print $"(ansi purple)max | gaming | g(ansi reset) ........... (ansi cyan)Max clocks/gaming profile"
	print "------------------------------------------------------------------------------------------------------------"
} 

export def mode-set [mode: string] {
	
	dependency_check "nvidia-smi"
	detect_os linux bsd

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
			get_help
			error make {
				msg: "Unknown Mode",
				label: {
					text: $"\"($mode)\" Is not a valid mode.",
					span: (metadata $mode).span
				},
				exit_code: 1
			}
		},
	}

	^(any_one_of sudo doas run0) nu --commands $"^nvidia-smi -pm 1; ^nvidia-smi ($NVIDIA_ARGS)" | ignore
}
