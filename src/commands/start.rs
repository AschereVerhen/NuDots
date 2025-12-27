use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Example, LabeledError, PipelineData, Signature};
use crate::{NuStartPlugin};
use crate::utils::writelogic::get_config;

pub struct Start;

impl PluginCommand for Start {
    type Plugin = NuStartPlugin;

    fn name(&self) -> &str {
        "nustart start"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
    }

    fn description(&self) -> &str {
        "NuStart Start: Start the commands in the database."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["database", "start"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        start(call)?;
        Ok(PipelineData::Empty)
    }
}


pub fn start(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    //First, lets import all the syscalls:
    use crate::syscalls::{
        execve::execve,
        setsid::Pid,
    };
    use crate::make_error;
    use std::ffi::CString;
    let mut config = get_config();
    let programs = config.get_programs_mut().iter();
    for program in programs {
        if ! program.get_enable() {continue}
        let restart = program.get_restart();
        let mut arguments = program.get_arguments();
        let name = program.get_name();
        //we must pass name into the arguments vector.
        arguments.insert(0, name);
        let safe_arguments: Vec<CString> = arguments.into_iter().map(|element| {
            match CString::new(element.clone()) {
                Ok(cstring) => cstring,
                Err(_) => panic!("The argument: {element}, contained a Null byte.")
            }
        }).collect();
        let envs: Vec<CString> = std::env::vars().map(|(key, value)| {
            CString::new(format!("{}={}", key, value)).unwrap()
        }).collect();
        let result = Pid::fork().map_err(|err| make_error!(
        format!("An Error Occured when forking: {:?}", err),
        "Failed to fork",
        call.head
        ))?.get_raw();
        if result == 0 {
            //Child pid. We will do setsid, execve, and then constantly restart the process if --restart was passed to it.
            Pid::setsid().unwrap();
            execve(
                safe_arguments.clone(),
                envs.clone(),
                call,
            )?;
        }
        //Else, we are the parent. In this case, we want to watch for the child, yk reap it.
        if restart {
            let pid = Pid::fork().unwrap().get_raw(); //we do not care about the parent in this case. we just want to loop in case of the child.
            if pid != 0 { continue; }
            if pid == 0 {
                let _ = Pid::setsid();
                loop {
                    let worker_pid = Pid::fork().unwrap().get_raw();
                    if worker_pid == 0 {
                        execve(safe_arguments.clone(), envs.clone(), call)?;
                    } else {
                        let _ = Pid::from(worker_pid).wait_for_child();
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                }
            }
        }
    }

    Ok(PipelineData::Empty) // Placeholder.
}