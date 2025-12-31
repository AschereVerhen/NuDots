use crate::prelude::*;


pub fn stop(pidunits: &Vec<PidUnit>,name_passed: Option<&String>, all: bool) -> Result<PipelineData, LabeledError> {
    let mut new_pidfile: Vec<PidUnit> = Vec::new();
    for pidunit in pidunits {
        let pid = pidunit.get_pid();
        let name = pidunit.get_name();
        let monitor = pidunit.get_monitor();
        if monitor {
            //First kill the monitors
            if let Some(name_dec) = name_passed && name == name_dec {
                pid.kill()?;
                continue;
            }
            if all {
                pid.kill()?;
                continue;
            }
            debugf!("A Monitor process survived. It means that either the user intended it to survive or a bug occured.");
            let saved_monitor_pid = PidUnit::new(pid.clone(), name.to_string(), monitor);
            new_pidfile.push(saved_monitor_pid);
            continue;
        }
        //Else do the same thing.
        if let Some(name_dec) = name_passed && name == name_dec {
            pid.kill()?;
            continue;
        }
        if all {
            pid.kill()?;
            continue;
        }
        //Now, if nothing happened, it means that pid survived. So we append it to new_pidfile
        let saved_child_pid = PidUnit::new(pid, name.to_string(), monitor);
        new_pidfile.push(saved_child_pid);
    }
    //And now run write_pids
    let pidfile = PidFile::new(new_pidfile);
    write_pid(pidfile).map_err(
        |e| {
            make_error!(
                format!("An Error occurred while writing to PID file: {:?}", e),
                "",
                nu_protocol::Span::unknown()
            )
        }
    )?;

    Ok(PipelineData::Empty)
}





pub fn call_stop(
    _: EngineInterface,
    call: EvaluatedCall,
    _: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let pids = get_pids(&call)?;
    let args: Vec<String> = call.rest(0)?;
    let all: bool = call.has_flag("all")?;
    //Using args.get() as it returns a Option<&String> and is safer.
    stop(&pids, args.get(0), all)?;
    Ok(PipelineData::Empty)
}

fn sig() -> Signature {
    Signature::build(Stop.name())
        .add_help()
        .rest(
            "Command's name",
            SyntaxShape::String,
            "the name of the command to stop. Only enter the first name."
        )
        .switch(
            "all",
            "Stop all running programs",
            Some('a')
        )
}

#[plugin_command(
name = "nustart stop",
plugin = NuStartPlugin,
description = "Easily stop a running background process with this.",
run = call_stop,
signature = sig()
)]
pub struct Stop;