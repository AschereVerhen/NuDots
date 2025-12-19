use nu_plugin::{
    PluginCommand,
};

use nu_protocol::{
    PipelineData, Signature, Value, Type,
};

use crate::Nudo;

pub struct Dev;

impl PluginCommand for Dev {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev"
    }
    fn description(&self) -> &str {
        "These are Developer commands. These are kinda not meant for normal use. but you are free to use em."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .input_output_type(Type::Any, Type::Nothing)
            .add_help()
    }
    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            nu_protocol::Example {
                example: "nudo dev anyoneof hyprland qs emerge",
                description: "Easily get which package you have installed.",
                result: Some(Value::test_string("emerge")),
            },
            nu_protocol::Example {
                example: "['waybar', 'startx', 'bluetoothctl'] | nudo dev dependcheck",
                description: "Also takes in from stdin.",
                result: Some(Value::test_string("bluetoothctl")),
            },
            nu_protocol::Example {
                example: "nudo dev dependcheck hyprland qs emerge",
                description: "Easily check your dependencies.",
                result: None,
            },
            nu_protocol::Example {
                example: "['waybar', 'startx', 'bluetoothctl'] | nudo dev dependcheck",
                description: "Also takes in from stdin.",
                result: None,
            },
            nu_protocol::Example {
                example: "nudo dev detectos",
                description: "Easily Know your distro or OS",
                result: Some(Value::test_string("Arch Linux"))
            },
            nu_protocol::Example {
                example: "nudo dev run echo -e \"Hello World!\n\"",
                description: "Run external commands seemlessly, without having to do any arguments gymnastics",
                result: Some(Value::test_string("Hello World!\n")),
            },
            nu_protocol::Example {
                example: "['echo', '-e', 'Hello World!\n'] | nudo dev run",
                description: "Also Run external commands from a list from stdin.",
                result: Some(Value::test_string("Hello World!\n"))
            }
        ]
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            _call: &nu_plugin::EvaluatedCall,
            _input: nu_protocol::PipelineData,
        ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        Ok(PipelineData::Empty)
    }
}