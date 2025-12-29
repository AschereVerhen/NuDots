use crate::{plugincmd, NuStartPlugin};
plugincmd!(
    plugin: NuStartPlugin,
    name: NuStart,
    cliName: "nustart",
    signature: {
        Signature::build(NuStart.name()).add_help()
    },
    description: r#"NuStart: An autostart manager served as a nushell plugin."#,
    searchTerms: ["nustart", "start", "autostart"],
    examples: [],
    run: |_,engine: &nu_plugin::EngineInterface, _call, _input| -> Result<PipelineData, nu_protocol::LabeledError> {
        println!("{}", engine.get_help()?);
        Ok(PipelineData::Empty)
    }
);