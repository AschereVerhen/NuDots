#not/usr/bin/env nu


##This function handles autostart of processes. The programs shall be set using nudo
##Example Usage: nudo set autostart <program+args> && nudo remove autostart <program+args/id> etc.

const utils = ($nu.default-config-dir | path join "functions/utils.nu")
use $utils [debug_print]


export def astart [] {
	if not ($nu.data-dir | path exists) {debug_print $"astart: Creating Directory: ($nu.data-dir).";mkdir ($nu.data-dir)};
	let lockfile_dir = "/tmp/nudo/astart";
	if not ($lockfile_dir | path exists) {debug_print $"astart: Creating Directory: ($lockfile_dir)";mkdir $lockfile_dir};
	let autostart_database = ($nu.data-dir | path join "astart-repo"); ##This is where the list of commands will be stored. More on this in aset.
	if not ($autostart_database | path exists) {debug_print $"astart: Creating a new autostart database at ($autostart_database)."; "" | save -f $autostart_database};
	let commands = (open $autostart_database | from json);
	#And then just make a background process for the program IFF it does not already work. We will create a lock for this. in /run/nushell/astart.
	$commands | each {
	|command|
		let lock_file = ($lockfile_dir | path join ($command | hash sha256))
		if ($lock_file | path exists) {
			#do nothing
			debug_print $"astart: Lockfile: ($lock_file) exists. Will Not start the command: ($command)."
			return
		} else {
			setsid ($command | str join ' ') | ignore
			debug_print $"astart: Spawned command: ($command)"
			touch $lock_file ##On reboot /tmp will be wiped... because tmpfs.
		}
	} | ignore
}

export def aget [] {
	#Firstly, ensure ($nu.data-dir) exists.
	if not ($nu.data-dir | path exists) {mkdir ($nu.data-dir)};
	#Then, lets get the database.
	let autostart_database = ($nu.data-dir | path join "astart-repo");
	if not ($autostart_database | path exists) {"" | save -f $autostart_database};
	#And now parsing game.
	open $autostart_database | from json
}


export def aset [command: list<string>] {
	#Firstly, ensure ($nu.data-dir) exists.
	if not ($nu.data-dir | path exists) {mkdir ($nu.data-dir)};
	#Then, lets get the database.
	let autostart_database = ($nu.data-dir | path join "astart-repo");
	#now, we write to this database.
	debug_print $"aset: adding Command: ($command | str join ' ')"
	
	debug_print "Deciding if the command is in autostart database"
	if ((open $autostart_database | is-not-empty) and (($command | str join ' ') in (open $autostart_database | from json))) {
		debug_print "The command is already in autostart database. Returning."
		return
	}
	debug_print "The command is not in autostart database. Adding."

	let new_database = open $autostart_database | from json | append ($command | str join ' ') | to json | save -f $autostart_database
	astart
}
def write-table [tb: list<string>] {
	if not ($nu.data-dir | path exists) {mkdir ($nu.data-dir)};
	#Then, lets get the database.
	let autostart_database = ($nu.data-dir | path join "astart-repo");
	debug_print $"write-table: old database: (open $autostart_database | str join '\n')"	
	#Then first erase the old file.
	"" | save -f $autostart_database

	#then we write the new table.
	let new_database = ($tb |  each {
		|cmd|
			if ($cmd != " ") {
				$"($cmd)\n" | save -af $autostart_database #Make sure to use -a. append.
			}
	})
	debug_print $"write-table: new database: ($new_database | str join '\n')"
}

export def adel [command_or_id: list<string>] {
	##Now, the command_or_id can either be a list of string or multiple numbers.
	let id_list = $command_or_id | each {|command|
		try {
			($command | into int)
		} catch {
			null
		}
	};
	debug_print $"adel: id_list: ($id_list)";
	let string_list = $command_or_id | each {|command| 
		try {
			($command | into int)
			null
		} catch {
			$command
		}
	} #The elements not in id_list will be strings.
	debug_print $"adel: string_list: ($string_list)"
	let commands = aget;
	debug_print $"adel: commands: ($commands)"
	let id_to_string = if ($id_list | is-not-empty) {
		$id_list | each {|index|
			if ($index < ($commands | length)) {
				($commands | get $index)
			}
	}} else {null};
	debug_print $"adel: id_to_string: ($id_to_string)"

	let new_commands = $commands | each {|command|
		debug_print $"adel: Current Command: ($command)"
		if not ($command in $string_list) {
			debug_print $"adel: ($command) is not in ($string_list)."
			if ($id_to_string | is-empty) {
				debug_print $"adel: $id_to_string is empty. Keeping ($command)."
				$command
			} else {
				if not ($command in $id_to_string) {
					debug_print $"($command) is not in ($id_to_string). Keeping."
					$command
				}
			}
		}
	};
	debug_print $"adel: new_commands: ($new_commands | to text)"
	write-table $new_commands
}

