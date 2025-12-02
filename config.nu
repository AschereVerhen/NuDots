###Setting Up Toggles File###
if not ($nu.data-dir | path exists) {
    mkdir $nu.data-dir
}
if not ($nu.data-dir | path join "toggles" | path exists) {
	"" | save -f ($nu.data-dir | path join "toggles")
}

###Sourcing Functions###
$env.PERSISTENT_TOGGLES = ($nu.data-dir | path join "toggles" | open | from json)
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
if (which starship | is-not-empty) {
	$env.PROMPT_COMMAND = {|| 
    		# The default Starship command for Nushell
    		starship prompt
	}
}
$env.config.show_banner = false
$env.PROMPT_INDICATOR = {|| " " }
$env.PROMPT_INDICATOR_VI = {|| " " }
###End of Section ###


###Pywal Colors###
let colors = ( 
    try {
        $env.PERSISTENT_TOGGLES | find "colors" | get value | get 0 | str trim | into bool
    } catch {
	false
    }
)
# Define the file path string first.
if ($colors and (which wal | is-not-empty) and not ((tty) =~ "tty")) {
	let user_wall_path = $env.PERSISTENT_TOGGLES | find "wallpath" | get value? | get 0?
	let wallpath = ( $user_wall_path | default --empty ( "~/Pictures/Wallpapers/.current_wallpaper" | path expand ))
	job spawn { ^setsid wal -tqi $wallpath | ignore } #dark magic. Basically we are using setsid which says to wall "run.", but it runs not in our tty. hence it cannot output anything to us. usefull for suppressing kitty's @kitty{"ok": true} json blob.
	notify-send --app-name="Pywal" --urgency=normal "Generating a color scheme..." "Please wait, pywal is generating a color scheme from the terminal..."
}
###End of section###


###AutoStarts###
if ($nu.os-info.name == "linux" or $nu.os-info.name =~ "bsd") {
	wm
}
if (which fastfetch | is-not-empty) { fastfetch --config examples/10 }
init-all ##Initialize the keybinds for fzf integration
astart
###End Of section###

# --- Aliases ---
alias ff = fastfetch --config examples/10
alias search = paru --noconfirm
alias bvum = nvim
alias nvum = nvim
alias poweroff = sudo systemctl poweroff -i
alias ka = killall

