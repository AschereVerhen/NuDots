use crate::prelude::*;
fn get(
    _: EngineInterface,
    _: EvaluatedCall,
    _: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let record = get_config().into();
    Ok(PipelineData::Value(record, None))
}

fn sig() -> Signature {
    Signature::build(Get.name())
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

#[plugin_command(
    name = "nustart get",
    plugin = NuStartPlugin,
    signature = sig(),
    description = "NuStart Get: Get the list of autostart programs in Database.",
    run = get,
)]
pub struct Get;