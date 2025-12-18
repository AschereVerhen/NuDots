use nu_plugin::{
    EvaluatedCall,
    EngineInterface,
    SimplePluginCommand
};
use nu_protocol::{Category, Signature, Value};
use crate::Nudo;
pub(crate) struct NudoDispatch;

impl SimplePluginCommand for NudoDispatch {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo"
    }
    fn description(&self) -> &str {
        "nudo: Your friendly sysadministration tool for nushell."
    }
    // fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        
    // }
    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .category(Category::Experimental)
    }

    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &EngineInterface,
            call: &EvaluatedCall,
            _input: &Value,
        ) -> Result<Value, nu_protocol::LabeledError> {
        Ok(Value::string("Hello World", call.head))
    }
}