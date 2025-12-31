use crate::commands::start::start;
use crate::prelude::*;
fn call_enable(
    _: EngineInterface,
    call: EvaluatedCall,
    _: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let now = call.has_flag("now")?;
    let commands: Vec<String> = call.rest(0)?;
    let index:Option<usize> = call.get_flag("index")?;
    let all = call.has_flag("all")?;
    enable(
        now,
        commands,
        index,
        all,
        call
    )?;
    Ok(PipelineData::Empty)
}

pub fn enable(
    now: bool,
    commands: Vec<String>,
    index: Option<usize>,
    all: bool,
    call: EvaluatedCall,
) -> Result<(), LabeledError> {
    let mut programs = get_config();
    let programs_vec = programs.get_programs_mut();
    let len = programs_vec.len();
    for (i,program) in programs_vec.iter_mut().enumerate() {
        if all {
            program.set_enabled();
            continue;
        }
        if let Some(name) = commands.first() && name == &program.get_name() {
            program.set_enabled();
        }
        if let Some(idx) = index && idx < len && i == idx {
            program.set_enabled();
        }
    }
    if now {
        start(programs_vec, &call)?;
    }
    write_configfile(programs).map_err(|e| {
        make_error!(format!("An error occured while writing the config file: {}", e), "", call.head)
    })?;
    Ok(())
}

fn sig() -> Signature {
    Signature::build(Enable.name())
        .add_help()
        .switch(
            "now",
            "Will Enable the command right now.",
            Some('n')
        )
        .rest(
            "Command",
            SyntaxShape::String,
            "The command to enable"
        )
        .named("index", SyntaxShape::Int, "The Index to look up and enable", Some('i'))
        .switch("all", "Enable all commands", Some('a'))
}

#[plugin_command(
    name = "nustart enable",
    plugin = NuStartPlugin,
    signature = sig(),
    description = "NuStart Enable: Enable a disabled program easily.",
    run = call_enable,
)]
pub struct Enable;