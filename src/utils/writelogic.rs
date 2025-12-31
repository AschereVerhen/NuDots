use std::path::PathBuf;
use directories::BaseDirs;
use crate::prelude::*;
fn get_configpath() -> PathBuf {
    let basedir = BaseDirs::new().expect("failed to get home directory");
    let config_path = basedir.data_dir().join("nustart").join("autostart.json");
    debugf!("Config path was found to be: {:?}", config_path);
    if let Some(parent) = config_path.parent() {
        debugf!("Creating directory {:?}", parent);
        std::fs::create_dir_all(parent).expect("failed to create config directory");
    }

    if !config_path.exists() {
        debugf!("Creating default config");
        std::fs::write(&config_path, r#"{"programs":[]}"#).expect("failed to create config file");
    }
    config_path
}

pub fn get_config() -> ConfigFile {
    let config_path = get_configpath();
    let data = std::fs::read_to_string(&config_path).expect("failed to read config file");
    debugf!("Getting the config file.");
    let deserialize: ConfigFile =
        serde_json::from_str(&data).expect("Error While parsing config file");
    deserialize
}

pub fn append_confunit(config_unit: ConfigUnit) -> std::io::Result<()> {
    use std::io::Write;
    let config_path = get_configpath();
    let mut new_config = get_config();
    //check if the current config is just the default value:
    let default_val = r#"{"programs":[]}"#;
    let serialize = serde_json::to_string(&new_config)?;
    let mut file = std::fs::File::options()
        .append(false)
        .write(true)
        .truncate(true)
        .create(false)
        .open(&config_path)?;
    //We need to check if the config file itself is empty; if it is we will overwrite it only
    // with the given ConfigUnit.
    if serialize == default_val /*Check if the serialized value is empty*/ {
        //create a new configfile with only the given ConfigUnit as its program.:
        debugf!("Got default config, writing to file without any appends.");
        let new_conf_file = ConfigFile::new(vec![config_unit]);
        let new_serialization = serde_json::to_string_pretty(&new_conf_file)?;
        writeln!(&mut file, "{}", new_serialization)?; //Now writing.
        return Ok(()); //Exit from the tree.
    }
    //Else, if its not empty, we will push the ConfigUnit given to the programs: field and push
    // the whole ConfigFile.
    debugf!("Writing to file {:?} with appends", &config_path);
    new_config.get_programs_mut().push(config_unit);
    let new_serialization = serde_json::to_string_pretty(&new_config)?;
    writeln!(&mut file, "{new_serialization}")?;

    Ok(())
}

pub fn write_configfile(config_file: ConfigFile) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(get_configpath())?;
    let serialize = serde_json::to_string(&config_file)?;
    writeln!(&mut file, "{serialize}")?;
    debugf!("Write Successful.");
    Ok(())
}
//PIDS are not supposed to be non-volatile.
pub fn destroy_pids() {
    let pid = get_pid_path();
    std::fs::remove_file(pid).expect("failed to remove pid file");
}

pub fn get_pid_path() -> PathBuf {
    let path = BaseDirs::new().expect("Failed to get home directory");
    let pid_dir = path.data_dir().join("nustart").join("pid.txt");
    if let Some(parent) = pid_dir.parent() {
        debugf!("Creating directory {:?}", parent);
        std::fs::create_dir_all(parent).expect("failed to create config directory");
    }

    if !pid_dir.exists() {
        debugf!("Creating default pid file(empty)");
        std::fs::write(&pid_dir, "").expect("Failed to create pid file");
    }

    pid_dir
}


pub fn write_pid(pid: PidFile) -> std::io::Result<()> {
    let path = get_pid_path();
    let mut file = std::fs::File::options().write(true).truncate(true).open(path)?;
    use std::io::Write;
    let content = serde_json::to_string_pretty(&pid)?;
    writeln!(&mut file, "")?; //nuke the file first.
    writeln!(&mut file, "{content}")?;
    Ok(())
}

pub fn get_pids(call: &EvaluatedCall) -> Result<Vec<PidUnit>, nu_protocol::LabeledError> {
    let pid_path = get_pid_path();
    let contents = std::fs::read_to_string(&pid_path).expect("failed to read pid file");
    let file: PidFile = serde_json::from_str(&contents)
        .map_err(|e| {
            use nu_protocol::LabeledError;
            make_error!(
                format!("An Error While parsing pid file: {}", e),
                "Maybe the file is invalid?",
                call.head
            )
        })?;
    Ok(file.get_pids())
}