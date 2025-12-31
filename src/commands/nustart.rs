use plugin_command_macro::plugin_command;
use crate::prelude::*;

fn start(engine: EngineInterface, _: EvaluatedCall, _: PipelineData) -> Result<PipelineData, nu_protocol::LabeledError> {
    println!("{}", engine.get_help()?);
    Ok(PipelineData::Empty)
}


#[plugin_command(
    plugin = NuStartPlugin,
    name = "nustart",
    signature = Signature::build(NuStart.name()).add_help(),
    description = "NuStart: An autostart manager served as a nushell plugin.",
    run = start,
)]
pub struct NuStart;