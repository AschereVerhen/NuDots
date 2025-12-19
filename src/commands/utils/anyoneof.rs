use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type, Value
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::Nudo;

pub struct AnyOneOf;
use crate::errors::MyError;
use which::which;
pub fn anyoneof_raw(check_deps: &Vec<String>) -> Result<String, MyError> {
    let value: Option<&String> = check_deps.into_iter().find(|program| which(program).is_ok());

    if value.is_none() {
        return Err(MyError::DependencyNotSatisfied)
    } else {
        return Ok(value.unwrap().clone())
    }
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
            .input_output_types(vec![
                (Type::list(Type::String), Type::Nothing),
                (Type::String, Type::Nothing),
                (Type::Nothing, Type::Nothing)
            ])
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
