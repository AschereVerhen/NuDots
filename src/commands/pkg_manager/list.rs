use crate::prelude::*;
pub fn list(call: &EvaluatedCall, packages: Vec<String>, os: OS, no_confirm: bool, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, no_confirm, PkgOp::ListInstalled)
}
fn signature() -> Signature {
    Signature::new(ListPkg.name())
        .category(Category::Custom("Package Management".to_string()))
        .add_help()
        .input_output_type(Type::Nothing, Type::Nothing) //Takes in anything; returns nothing.
        .allows_unknown_args() //Allow people to pass pkg_manager-specific flags, like --one-shot in emerge or --overwrite="*" in pacman.
}
fn call_list(
    engine: nu_plugin::EngineInterface,
    call: EvaluatedCall,
    _input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let no_confirm:bool  = call.has_flag("yes")?;
    let os = detect_os_raw();
    list(&call, vec![], os, no_confirm, &engine)?;
    Ok(PipelineData::Empty)
}
#[plugin_command(
name = "nupkg list",
description = "Allows you to list all the packages installed on your system os-agnostically",
plugin = Nudo,
signature = signature(),
run = call_list
)]
pub struct ListPkg;