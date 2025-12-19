use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type
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
        let c: Vec<&str> = cmd.split_whitespace().collect(); //If the cmd itself is: "/usr/bin/foo bar bazz", convert it into "/usr/bin/foo", ["foo", "bar"].
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
            .optional("Command", SyntaxShape::String, "The Command to execute")
            .rest(
                "Arguments",
                SyntaxShape::String,
                "The list of arguments to run the command with."
            )
            .add_help()
            .allows_unknown_args()
            .input_output_types(vec![
                (Type::list(Type::String), Type::Nothing),
                (Type::String, Type::Nothing),
                (Type::Nothing, Type::Nothing)
            ])
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
        let mut args_stdin = Vec::new();
        for val in input {
            args_stdin.push(val.as_str()?.to_string())
        }
        let cmd_opt = call.opt(0)?;
        let command = if args_stdin.len() == 0 && cmd_opt.is_some() {cmd_opt.unwrap()} else {args_stdin[0].clone()};
        let arguments = if args_stdin.len() == 0 {call.rest(1)?} else {args_stdin[1..].to_vec()};
        // let command = call.req(0).unwrap_or(args_stdin[0].clone());
        // let arguments = call.rest(1).unwrap_or(args_stdin[1..].to_vec());
        run(call, command, arguments)?;
        Ok(PipelineData::Empty)
    }
}