use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, Value, Type
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::Nudo;

pub struct DetectOs;

pub fn detect_os_raw() -> Result<String, Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;
    if os != "linux" {
        return Ok(os.to_string());
    }
    //Now, we first of all open /etc/os-release now that we *know* the system is linux.
    let mut file = std::fs::File::open("/etc/os-release")?;
    let mut contents = String::new();
    use std::io::Read;
    let _ = file.read_to_string(&mut contents)?;
    let distro: Option<String> = contents.lines().find(|line| line.starts_with("ID=")).and_then(|line| line.split("=").nth(1)).map(|s| s.trim_matches('"').to_string());
    Ok(distro.unwrap_or("Unknown Linux".to_string()))
}

fn detect_os(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let os = detect_os_raw().map_err(|e| LabeledError::new(e.to_string()))?;
    return Ok(PipelineData::value(Value::string(os, call.head), None));
}

impl PluginCommand for DetectOs {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev detectos"
    }
    fn description(&self) -> &str {
        "This command detects and returns Your Operating System. And if its Linux Or BSD, it will also return the Distro."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .add_help()
            .input_output_type(Type::Nothing, Type::Nothing)
            .category(Category::Custom("Developer".to_string()))
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
            detect_os(call)
    }
}
