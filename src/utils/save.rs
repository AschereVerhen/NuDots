//NOTE: THIS FILE WILL NOT BE A NUSHELL MODULE.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use nu_protocol::{record, Span, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigUnit {
    pub name: String,
    arguments: Vec<String>,
    path: PathBuf,
    restart: bool,
    enable: bool,
    //This is enough for now.
}

impl ConfigUnit {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn get_restart(&self) -> bool {
        self.restart
    }
    pub fn get_enable(&self) -> bool {
        self.enable
    }
    pub fn new(name: String, arguments: Vec<String>, restart: bool, enable: bool) -> Self {
        let path = match which::which(&name) {
            Ok(path) => path,
            Err(e) => panic!(
                "Failed to get path of the program: {}. Error: {:?}",
                &name, e
            ),
        };
        Self {
            name,
            arguments,
            path,
            restart,
            enable
        }
    }
}

//Implimenting from<ConfigUnit> for Values
impl From<ConfigUnit> for Value {
    fn from(confunit: ConfigUnit) -> Value {
        let span = Span::unknown();
        Value::record(
            record!(
                "name" => Value::string(confunit.get_name(), span),
                "arguments" => Value::list(confunit.get_arguments()
                    .into_iter()
                    .map(|x| Value::string(x, span))
                    .collect::<Vec<Value>>(), span),
                "path" => Value::string(confunit.get_path().to_string_lossy(), span),
                "restart" => Value::bool(confunit.get_restart(), span),
                "enable" => Value::bool(confunit.get_enable(), span),
            ),
            span
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFile {
    programs: Vec<ConfigUnit>
}

impl ConfigFile {
    pub fn new(programs: Vec<ConfigUnit>) -> Self {
        Self { programs }
    }
    pub fn get_programs(&self) -> &Vec<ConfigUnit> {
        &self.programs
    }
    pub fn get_programs_mut(&mut self) -> &mut Vec<ConfigUnit> {
        &mut self.programs
    }
}

impl From<ConfigFile> for Value {
    fn from(conf: ConfigFile) -> Value {
        let span = Span::unknown();
        let table: Vec<Value> = conf.programs.into_iter().map(Value::from).collect();
        Value::record(
            record!(
                "programs" => Value::list(table, span),
            ),
            span
        )
    }
}


