use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::Nudo;

pub struct Run;
//No Need to impliment raw version of this. This is all good.
pub fn run(call: &EvaluatedCall, cmd: String, arguments: Vec<String>) -> Result<(), LabeledError> {
    use std::process::{Command, Stdio};
    let args;
    let mut command_raw: String = cmd.to_string();
    if arguments.len() != 0 {
        args = arguments
    } else {
        let c: Vec<&str> = cmd.split_whitespace().collect();
        command_raw = c[0].to_string();
        args = c[1..].into_iter().map(|element| element.to_string()).collect::<Vec<String>>();
    }
    let mut command = Command::new(command_raw);
    command.args(args);
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.stderr(Stdio::inherit());
    let status = command.status().map_err(|e| LabeledError::new(e.to_string()).with_label("Failed to run Command", call.head))?;

    if !status.success() {
        return Err(
            LabeledError::new("Command failed with an non-zero Exit code.")
                .with_label("Something wrong happened", call.head)
        )
    }

    return Ok(())
}

impl PluginCommand for Run {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev run"
    }
    fn description(&self) -> &str {
        "Runs a command with an args list. And then Provides a labelled Error if it errored out."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Developer".to_string()))
            .required("Command", SyntaxShape::String, "The Command to execute")
            .rest(
                "Arguments",
                SyntaxShape::String,
                "The list of arguments to run the command with."
            )
            .add_help()
            .allows_unknown_args()
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
        let command = call.req(0)?;
        let arguments = call.rest(1)?;
        run(call, command, arguments)?;
        Ok(PipelineData::Empty)
    }
}