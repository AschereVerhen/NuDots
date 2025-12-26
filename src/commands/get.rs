use nu_plugin::{PluginCommand, EngineInterface, EvaluatedCall};

use nu_protocol::{Signature, Example, PipelineData, LabeledError, Type};
use crate::{mybox, NuStartPlugin};
use crate::utils::writelogic::{get_config};

pub struct Get;

impl PluginCommand for Get {
    type Plugin = NuStartPlugin;

    fn name(&self) -> &str {
        "nustart get"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .add_help()
            .input_output_type(Type::Nothing, Type::Record(
                mybox!([("programs".to_string(), Type::Table(mybox!([
                    ("name".to_string(), Type::String),
                    ("arguments".to_string(), Type::List(Box::new(Type::String))),
                    ("path".to_string(), Type::String),
                    ("restart".to_string(), Type::Bool),
                    ("enabled".to_string(), Type::Bool),
                ])))])
            ))
    }

    fn description(&self) -> &str {
        "NuStart Add: Add a command to autostart."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["enable", "save", "add"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        _call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let record = get_config().into();
        Ok(PipelineData::Value(record, None))
    }
}
