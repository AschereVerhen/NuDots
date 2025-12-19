use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type, Value
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::Nudo;

pub struct AnyOneOf;
use crate::commands::utils::dependency::{get_path_resolved_raw, get_bins_raw};
use crate::errors::MyError;
pub fn anyoneof_raw(check_deps: &Vec<String>) -> Result<String, MyError> {
    let path = get_path_resolved_raw()?;
    // dbg!("Got path: {:?}", &path);
    let bins = get_bins_raw(path)?;
    // dbg!("Got bins: {:?}", &bins);
    for dep in check_deps.iter() {
        match bins.contains(dep) {
            true => {
                // dbg!("Path contains: {}", dep);
                return Ok(dep.clone())
            }
            false => {
                // dbg!("Path does not contain: {}", dep);
                continue;
            }
        }
    }
    return Err(MyError::DependencyNotSatisfied)
}

pub fn anyoneof(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let deps: Vec<String> = call.rest(0)?;
    let value = anyoneof_raw(&deps);
    match value {
        Ok(dep) => {
            return Ok(PipelineData::value(Value::string(dep, call.head), None))
        },
        Err(MyError::DependencyNotSatisfied) => {
            return Err(LabeledError::new("Dependencies Not Satified.")
                .with_label(format!("None of the following: {} programs were installed", deps.join(", ")), call.head)
                .with_help("Install any one of the programs."))
        },
        _ => return Err(LabeledError::new("Unknown Error Occured")),
    }
}


impl PluginCommand for AnyOneOf {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev anyoneof"
    }
    fn description(&self) -> &str {
        "Note: This is a developer command\nThis function is there to get any one of the commands listed. It takes in a list of names, gets path, and returns the first program that is in the path from the list."
    }
    fn signature(&self) -> nu_protocol::Signature {
        Signature::new(self.name())
            .category(Category::Custom("Developer".to_string()))
            .input_output_type(Type::list(Type::String), Type::String)
            .add_help()
            .rest(
                "Dependencies",
                SyntaxShape::String,
                "List of Optional Dependencies"
            )
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
            anyoneof(call)
        }
}
