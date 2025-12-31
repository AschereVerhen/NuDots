use crate::prelude::*;

fn signature() -> Signature {
    Signature::build(Add.name())
        .required("Command", SyntaxShape::String, "Command to add")
        .rest(
            "param",
            SyntaxShape::String,
            "Additional Parameters to pass to the command",
        )
        .switch(
            "restart",
            "Whether to restart the command or not",
            Some('r'),
        )
        .add_help()
        .input_output_types(vec![
            (Type::String, Type::Nothing), //Maybe the user is passing through pipeline
            (Type::Nothing, Type::Nothing), //Maybe the user is passing through the cli
        ])
        .allows_unknown_args()
}
pub fn add(arguments: Vec<String>, restart: bool) -> Result<PipelineData, LabeledError> {
    let program = arguments[0].clone();
    let arguments = arguments[1..].to_vec();
    let confunit = ConfigUnit::new(program, arguments, restart, true)?;
    append_confunit(confunit).map_err(|e| LabeledError::new(e.to_string()))?; //This will append the
    // config unit to the autostart database.
    Ok(PipelineData::Empty)
}
pub fn call_add(engine: EngineInterface, call: EvaluatedCall, input: PipelineData) -> Result<PipelineData, LabeledError> {
    let program: String = call.req(0)?;
    let mut arguments: Vec<String> = call.rest(1)?;
    arguments.insert(0, program);
    let restart = call.has_flag("restart")?;
    if engine.is_using_stdio() {
        for arg in input {
            arguments.push(arg.as_str()?.to_string());
        }
    }
    add(arguments, restart)?;
    Ok(PipelineData::Empty)
}

#[plugin_command(
name = "nustart add",
plugin = NuStartPlugin,
signature = signature(),
description = "Add a command to autostart.",
run = call_add,
)]
pub struct Add;