use std::path::PathBuf;
use directories::BaseDirs;
use crate::utils::save::{ConfigFile, ConfigUnit};

fn get_configpath() -> PathBuf {
    let basedir = BaseDirs::new().expect("failed to get home directory");
    let config_path = basedir.data_dir().join("nustart").join("autostart.json");

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create config directory");
    }

    if !config_path.exists() {
        std::fs::write(&config_path, r#"{"programs":[]}"#).expect("failed to create config file");
    }
    config_path
}

pub fn get_config() -> ConfigFile {
    let config_path = get_configpath();
    let data = std::fs::read_to_string(&config_path).expect("failed to read config file");
    println!("NOW GETTING CONFIG FILE.");
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
        let new_conf_file = ConfigFile::new(vec![config_unit]);
        let new_serialization = serde_json::to_string_pretty(&new_conf_file)?;
        writeln!(&mut file, "{}", new_serialization)?; //Now writing.
        return Ok(()); //Exit from the tree.
    }
    //Else, if its not empty, we will push the ConfigUnit given to the programs: field and push
    // the whole ConfigFile.
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
    
    Ok(())
}