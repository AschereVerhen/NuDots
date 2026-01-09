use crate::prelude::*;
pub fn clean(call: &EvaluatedCall, packages: Vec<String>, os: OS, no_confirm: bool, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, no_confirm, PkgOp::CleanCache)
}

fn signature() -> Signature {
    Signature::new(Clean.name())
        .category(Category::Custom("Package Management".to_string()))
        .add_help()
        .switch("yes", "Skip Confirmation", Some('y'))
        .input_output_type(Type::Any, Type::Nothing) //Takes in anything; returns nothing.
        .allows_unknown_args() 
}

fn call_clean(
    engine: nu_plugin::EngineInterface,
    call: EvaluatedCall,
    _input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let no_confirm:bool  = call.has_flag("yes")?;
    let os = detect_os_raw();
    clean(&call, vec![], os, no_confirm, &engine)?;
    Ok(PipelineData::Empty)
}


#[plugin_command(
name = "nupkg clean",
description = "Allows you to clean your system os-agnostically",
plugin = Nudo,
run = call_clean,
signature = signature(),
)]
pub struct Clean;