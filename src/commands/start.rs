// use nu_protocol::LabeledError;
use crate::prelude::*;
#[plugin_command(
name = "nustart start",
plugin = NuStartPlugin,
description = "NuStart Start: Start the commmands in the database.",
signature = Signature::build(Start.name()).add_help(),
run = call_start
)]
pub struct Start;

fn call_start(
    _: EngineInterface,
    call: EvaluatedCall,
    _: PipelineData
) -> Result<PipelineData, LabeledError> {
    let mut config = get_config();
    let programs = config.get_programs_mut();
    start(programs, &call)?;
    Ok(PipelineData::Empty)
}


pub fn start(programs: &mut Vec<ConfigUnit>, call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    //First, lets import all the syscalls:
    use crate::syscalls::{execve::execve, setsid::Pid};
    use crate::make_error;
    use std::ffi::CString;
    for program in programs.iter() {
        //if the program is not enabled, we will just skip everything.
        if !program.get_enable() {
            continue;
        }
        let restart = program.get_restart();
        let mut arguments = program.get_arguments();
        let name = program.get_name();
        //we must pass name into the arguments vector.
        arguments.insert(0, name.clone());
        let safe_arguments: Vec<CString> = arguments
            .into_iter()
            .map(|element| match CString::new(element.clone()) {
                Ok(cstring) => cstring,
                Err(_) => {
                    panic!("The argument: {element}, contained a Null byte.");
                }
            })
            .collect();
        let envs: Vec<CString> = std::env::vars()
            .map(|(key, value)| CString::new(format!("{}={}", key, value)).unwrap())
            .collect();
        let result = Pid::fork()
            .map_err(|err| {
                make_error!(
                    format!("An Error Occured when forking: {:?}", err),
                    "Failed to fork",
                    call.head
                )
            })?
            .get_raw();
        if result == 0 {
            //Child pid. We will do setsid, execve, and then constantly restart the process if --restart was passed to it.
            Pid::setsid().unwrap();
            execve(safe_arguments.clone(), envs.clone(), &call)?;
        }
        //Else, we are the parent. In this case, we want to watch for the child, yk reap it.

        if restart {
            //we do not care about the parent in this case. we just want to loop in case of the child.
            let pid = Pid::fork().unwrap().get_raw();
            if pid != 0 {
                //write the pid of this monitor child process to out pids first before writing the og pid one.
                let monitor_pid = Pid::from(pid);
                let monitor = true;
                let name = name.clone();
                let pid = PidFile::new(
                    vec![
                        PidUnit::new(monitor_pid, name, monitor)
                    ]
                );
                write_pid(pid).map_err(|e| make_error!(format!("An Error occured when writing pid: {e:?}"), "Failed to write", call.head))?;
                continue;
            }
            if pid == 0 {
                let _ = Pid::setsid();
                loop {
                    let worker_pid = Pid::fork().unwrap().get_raw();
                    if worker_pid == 0 {
                        execve(safe_arguments.clone(), envs.clone(), &call)?;
                    } else {
                        let _ = Pid::from(worker_pid).wait_for_child();
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                }
            }
        }
        //Now write the pid
        let child_pid = Pid::from(result);
        let monitor = false;
        let name = name.clone();
        let pid = PidFile::new(
            vec![
                PidUnit::new(child_pid, name, monitor)
            ]
        );
        debugf!("{:?}", &pid);
        write_pid(pid).map_err(|e| make_error!(format!("An Error occured when writing pid: {e:?}"), "Failed to write", call.head))?;
    }

    Ok(PipelineData::Empty)
}
