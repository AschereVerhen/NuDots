use nu_plugin::PluginCommand;
use nu_protocol::{
    PipelineData,
    Signature,
    Example,
};

use crate::NuStartPlugin;

pub struct NuStart;

impl PluginCommand for NuStart {
    type Plugin = NuStartPlugin;

    fn name(&self) -> &str {
        "nustart"
    }

    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .add_help()
    }

    fn description(&self) -> &str {
        r#"NuStart: An autostart manager served as a nushell plugin."#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["nustart", "start", "autostart"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        _call: &nu_plugin::EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        println!("{}", _engine.get_help()?); // placeholder
        Ok(PipelineData::Empty)
    }
}
