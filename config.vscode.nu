###Setting Up Toggles File###
if not ($nu.data-dir | path exists) {
    mkdir $nu.data-dir
}
if not ($nu.data-dir | path join "toggles" | path exists) {
	"" | save -f ($nu.data-dir | path join "toggles")
}

###Sourcing Functions###
$env.PERSISTENT_TOGGLES = ($nu.data-dir | path join "toggles" | open | parse "{toggle}: {value}")
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


###AutoStarts###
if (which fastfetch | is-not-empty) { fastfetch --config examples/10 }
init-all ##Initialize the keybinds for fzf integration
###End Of section###

# --- Aliases ---
alias ff = fastfetch --config examples/10
alias search = paru --noconfirm
alias bvum = nvim
alias nvum = nvim
alias poweroff = sudo systemctl poweroff -i
alias ka = killall
