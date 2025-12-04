#!/usr/bin/env nu

use ($nu.default-config-dir | path join "functions/utils.nu") [dependency_check, detect_os, any_one_of, debug_print]
use ($nu.default-config-dir | path join "functions/settings.nu") [get-toggle]

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
			let saving_clocks = (get-toggle | find -i powersave | get value? | get 0?)
			if ($saving_clocks | is-empty) {
				error make {
					msg: $"(ansi red)Please set a powersaving mode using nudo set toggle.",
					label: {
						text: "This command returned null.",
						span: (metadata $saving_clocks).span,
					},
					exit_code: 1
				}
			}
			$"-lgc ($saving_clocks)"
			debug_print $"Got clocks: powersave mode: ($saving_clocks)"
		},
		"balanced" | "b" => {
			let balanced_clocks = (get-toggle | find -i balanced | get value? | get 0?)
			if ($balanced_clocks | is-empty) {
				error make {
					msg: $"(ansi red)Please set a balanced mode using nudo set toggle.",
					label: {
						text: "This command returned null.",
						span: (metadata $balanced_clocks).span,
					},
					exit_code: 1
				}
			}
			$"-lgc ($balanced_clocks)"
			debug_print $"Got clocks: balanced mode: ($balanced_clocks)"

		},
		"max" | "gaming" | "g" => {
			let max_clocks = (get-toggle | find -i gaming | get value? | get 0?)
			if ($max_clocks | is-empty) {
				error make {
					msg: $"(ansi red)Please set a 'gaming' mode using nudo set toggle.",
					label: {
						text: "This command returned null.",
						span: (metadata $max_clocks).span,
					},
					exit_code: 1
				}
			}
			$"-lgc ($max_clocks)"
			debug_print $"Got clocks: gaming mode: ($max_clocks)"
		},
		_ => {
			debug_print $"Mode: ($mode) not recognised."
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
