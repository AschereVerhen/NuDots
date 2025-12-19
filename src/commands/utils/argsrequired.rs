use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::Nudo;
//No need to impliment the raw version of this aswell.
pub fn args_required(call: &EvaluatedCall, min_args: u16) -> Result<(), LabeledError> {
    let arglist: Vec<String> = call.rest(0)?;
    if arglist.len() < min_args.into() {
        return Err(
            LabeledError::new("Minimum Args not matched.")
                .with_label(format!("Required Args: {}, Found args: {}", min_args, arglist.len()), call.head)
        )
    }
    return Ok(())

}

pub struct ArgsRequired;

impl PluginCommand for ArgsRequired {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev args_required"
    }
    fn description(&self) -> &str {
        "This command detects if the user supplied enough args or not. If not, it errors out early."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Developer".to_string()))
            .required("Arg", SyntaxShape::Int, "The Minimum Number of Args")
            .add_help()
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
        let min_args = call.req(0)?;
        args_required(call, min_args)?;
        Ok(PipelineData::Empty)
    }
}


