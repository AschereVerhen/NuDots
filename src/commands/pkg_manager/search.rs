use crate::prelude::*;
pub fn search(call: &EvaluatedCall, packages: Vec<String>, os: OS, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, false, PkgOp::Search)

}
fn signature() -> Signature {
    Signature::new(Search.name())
        .category(Category::Custom("Package Management".to_string()))
        .add_help()
        .input_output_type(Type::Any, Type::Nothing) //Takes in anything; returns nothing.
        .allows_unknown_args() //Allow people to pass pkg_manager-specific flags, like --one-shot in emerge or --overwrite="*" in pacman.
        .named(
            "Search Term",
            SyntaxShape::String,
            "The term to search for",
            Some('s')
        )
}
fn call_search(
    engine: EngineInterface,
    call: EvaluatedCall,
    _input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let search_term: String = call.req(0)?;

    let os = detect_os_raw();
    search(&call, vec![search_term], os, &engine)?;
    Ok(PipelineData::Empty)
}
#[plugin_command(
name = "nupkg search",
plugin = Nudo,
description = "Search packages using nupkg distro-agnostically",
signature = signature(),
run = call_search
)]
pub struct Search;