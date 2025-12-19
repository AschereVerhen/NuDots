use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};
use which::which;

use crate::{Nudo};
//Firstly, implimenting the dependency_check program.
//We do not want to add this to any nu api. Except error making.

//firstly, get path variable

pub fn dependcheck(call: &EvaluatedCall, input: PipelineData) -> Result<PipelineData, LabeledError> {
    let mut deps: Vec<String> = call.rest(0)?;
    for val in input {
        deps.push(val.as_str()?.to_string())
    }
    let mut not_found: Vec<String> = Vec::new();
    for dep in deps {
        if ! which(&dep).is_ok() {
            not_found.push(dep)
        }
    }
    if not_found.is_empty() {
        return Ok(PipelineData::Empty)
    } else {
        return Err(
            LabeledError::new("Dependency Check unsatisfied.")
                .with_label(format!("Dependencies Not found: {}", not_found.join(", ")), call.head)
                .with_help("Please install the above commands.")
        )
    }
}

pub struct DependencyCheck;

impl PluginCommand for DependencyCheck {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev dependcheck"
    }
    fn description(&self) -> &str {
        "This subcommand is there to ensure dependency check. It takes in a list of names, gets path, and ensures that the command is in path."
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
            "The dependencies to check."
        )
    }
    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            nu_protocol::Example {
                example: "nudo dev dependcheck hyprland qs emerge",
                description: "Easily check your dependencies.",
                result: None,
            },
            nu_protocol::Example {
                example: "['waybar', 'startx', 'bluetoothctl'] | nudo dev dependcheck",
                description: "Also takes in from stdin.",
                result: None,
            }
        ]
    }
    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        dependcheck(call, input)
    }
}