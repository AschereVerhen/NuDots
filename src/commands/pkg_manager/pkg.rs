use nu_plugin::{
    PluginCommand,
};

use nu_protocol::{
    PipelineData, Signature, Type,
};

use crate::Nudo;

pub struct Pkg;

impl PluginCommand for Pkg {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo pkg"
    }
    fn description(&self) -> &str {
        "This is the pkg module. This handles everything related to package management."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .input_output_type(Type::Any, Type::Nothing)
            .add_help()
    }
    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            nu_protocol::Example {
                example: "nudo pkg install hyprland qs emerge",
                description: "Easily install packages without having to memorize your distro's flags.",
                result: None,
            },
            nu_protocol::Example {
                example: "['waybar', 'startx', 'bluetoothctl'] | nudo pkg install",
                description: "Also takes in from stdin.",
                result: None,
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