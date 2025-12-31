use crate::commands::stop::stop;
use crate::prelude::*;
fn call_disable(
    _: EngineInterface,
    call: EvaluatedCall,
    _: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let now = call.has_flag("now")?;
    let commands: Vec<String> = call.rest(0)?;
    let index:Option<usize> = call.get_flag("index")?;
    let all = call.has_flag("all")?;
    disable(
        now,
        commands,
        index,
        all,
        call
    )?;
    Ok(PipelineData::Empty)
}

pub fn disable(
    now: bool,
    commands: Vec<String>,
    index: Option<usize>,
    all: bool,
    call: EvaluatedCall,
) -> Result<(), LabeledError> {
    let mut programs = get_config();
    let programs_vec = programs.get_programs_mut();
    let len = programs_vec.len();
    for (idx, program) in programs_vec.iter_mut().enumerate() {
        if all {
            program.set_disabled();
        }
        if let Some(name) = commands.first() && name == &program.get_name() {
            program.set_disabled();
        }
        if let Some(i) = index && i < len && i == idx {
            program.set_disabled();
        }
        if now {
            let pid_vec = get_pids(&call)?;
            for pid in pid_vec.iter() {
                if pid.get_name() == program.get_name() {
                    stop(&pid_vec, Some(&program.get_name()), false)?;
                }
            }
        }
    }
    write_configfile(programs).map_err(|e| {
        make_error!(format!("An error occured while writing the config file: {}", e), "", call.head)
    })?;
    Ok(())
}

fn sig() -> Signature {
    Signature::build(Disable.name())
        .add_help()
        .switch(
            "now",
            "Will disable the command right now.",
            Some('n')
        )
        .rest(
            "Command",
            SyntaxShape::String,
            "The command to disable"
        )
        .named("index", SyntaxShape::Int, "The Index to look up and disable", Some('i'))
        .switch("all", "disable all commands", Some('a'))
}

#[plugin_command(
    name = "nustart disable",
    plugin = NuStartPlugin,
    signature = sig(),
    description = "NuStart disable: disable a disabled program easily.",
    run = call_disable,
)]
pub struct Disable;