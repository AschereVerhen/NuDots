use nu_plugin::{
    EvaluatedCall,
    EngineInterface,
    PluginCommand
};
use nu_protocol::{Category, PipelineData, Signature, Type};
use crate::Nudo;
pub(crate) struct NudoDispatch;

impl PluginCommand for NudoDispatch {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nupkg"
    }
    fn description(&self) -> &str {
        "nupkg: Your friendly package manager for nushell.(linux only)"
    }
    // fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        
    // }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .category(Category::Experimental)
            .input_output_type(Type::Any, Type::Nothing)
            .add_help()
    }

    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &EngineInterface,
            _call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, nu_protocol::LabeledError> {
            println!("{}", _engine.get_help()?);
            Ok(PipelineData::Empty)
    }
}
