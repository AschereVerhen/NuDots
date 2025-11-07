###Sourcing Functions###
$env.PERSISTENT_CONFIG = ("~/.local/share/nushell" | path expand)
$env.PERSISTENT_TOGGLES = ($env.PERSISTENT_CONFIG | path join "toggles" | open | parse "{toggle}: {value}")
if not ($env.PERSISTENT_CONFIG | path exists) {
    mkdir $env.PERSISTENT_CONFIG
}
const functions_path = ($nu.config-path | path dirname | path join "functions")
if not ($functions_path | path exists) {
    mkdir $functions_path
    print "Nu shell scripts are to be stored at:" $functions_path
}
if not ($functions_path | path join "mod.nu" | path exists) {
    touch ($functions_path | path join "mod.nu")
}
use $functions_path *

###End of function section###


###Nushell Variables###
$env.PROMPT_COMMAND = {|| 
    # The default Starship command for Nushell
    starship prompt
}
$env.config.show_banner = false
# Starship handles the PROMPT_INDICATOR/PROMPT_INDICATOR_VI as well, 
# but setting them to a simple value is safe.
# Note the change from 'let-env PROMPT_INDICATOR' to '$env.PROMPT_INDICATOR'
$env.PROMPT_INDICATOR = {|| " " }
$env.PROMPT_INDICATOR_VI = {|| " " }
###End of Section ###


###Pywal Colors###
let colors = ( 
    try {
        $env.PERSISTENT_TOGGLES | find "colors" | get value | get 0 | str trim | into bool
    } catch {
        # true
	false
    }
)
# Define the file path string first.
if ($colors) {
	let wallpath = ( "~/Pictures/Wallpapers/.current_wallpaper" | path expand )
	wal -tqi /home/aschere/Pictures/Wallpapers/.current_wallpaper | ignore
}
###End of section###


###AutoStarts###
wm hyprland
fastfetch --config examples/10
###End Of section###

# --- Aliases ---
alias ff = fastfetch --config examples/13
alias search = paru --noconfirm
alias bvum = nvim
alias nvum = nvim
alias poweroff = sudo systemctl poweroff -i
