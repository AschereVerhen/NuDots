use crate::{mybox, plugincmd, NuStartPlugin};
use crate::utils::writelogic::{get_config};


plugincmd!(
    plugin: NuStartPlugin,
    name: Get,
    cliName: "nustart get",
    signature: {
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
    },
    description: "NuStart Add: Add a command to autostart.",
    searchTerms: ["enable", "save", "add"],
    examples: [],
    run: |_,_,_,_| -> Result<PipelineData, LabeledError> {
        let record = get_config().into();
        Ok(PipelineData::Value(record, None))
    }
);