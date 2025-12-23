use std::os::unix::process::CommandExt;

use nu_protocol::{
    Category, Example, LabeledError, PipelineData, Signature, SyntaxShape, Type, Value
};
use nu_plugin::{
    EngineInterface, EvaluatedCall, PluginCommand
};

use crate::Nudo;

pub struct Run;
//No Need to impliment raw version of this. This is all good.
pub fn run(call: &EvaluatedCall, cmd: String, arguments: Vec<String>, engine: &EngineInterface) -> Result<(), LabeledError> {
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
    
    unsafe {
        command.pre_exec(move || {
            use crate::syscalls::{setpgid::setpgid, kill::Pid};
            let pid = Pid::from_raw(0);
            let pgid = Pid::from_raw(0);
            setpgid(pid, pgid).unwrap();

            Ok(())
        });
    }
    let mut spawned = command
        .spawn()
        .map_err(|e| LabeledError::new(e.to_string())
            .with_label("Failed to spawn command", call.head))?;
    let pid = spawned.id();
    let _signal_guard = engine.register_signal_handler(Box::new({
        move |_| {
            use crate::syscalls::kill::{Pid, killpg, Signals};
            let pid = Pid::from_raw( pid as i32);
            let sig = Signals::SIGINT;
            //We want to kill the whole process group. Hence using killpg.
            killpg(
                pid,
                sig,
            ).unwrap()
        }
    }));
    spawned.wait().unwrap();

    return Ok(())
}

impl PluginCommand for Run {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudev run"
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
    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            Example {
                example: "nudo dev run echo -e \"Hello World!\n\"",
                description: "Run external commands seemlessly, without having to do any arguments gymnastics",
                result: Some(Value::test_string("Hello World!\n")),
            },
            Example {
                example: "['echo', '-e', 'Hello World!\n'] | nudo dev run",
                description: "Also Run external commands from a list from stdin.",
                result: Some(Value::test_string("Hello World!\n"))
            }
        ]
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            engine: &nu_plugin::EngineInterface,
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
        run(call, command, arguments, engine)?;
        Ok(PipelineData::Empty)
    }
}