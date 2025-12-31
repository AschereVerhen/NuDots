use nu_protocol::{LabeledError};
use serde::{Deserialize, Serialize};
use syscalls::{syscall, Errno, Sysno};
use crate::make_error;

#[allow(non_camel_case_types)]
pub type pid_t = u32;
#[derive(Serialize, Deserialize, Debug, Clone)] //Cloning is cheap here.
pub struct Pid {
    pid: pid_t,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PidUnit {
    pid: Pid,
    name: String,
    monitor: bool,
}

impl PidUnit {
    pub fn new(pid: Pid, name: String, monitor: bool) -> Self {
        Self { pid, name, monitor }
    }
    pub fn get_pid(&self) -> Pid {
        self.pid.clone()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_monitor(&self) -> bool {
        self.monitor
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PidFile {
    pids: Vec<PidUnit>
}

impl PidFile {
    pub fn new(pids: Vec<PidUnit>) -> Self {
        Self { pids }
    }
    pub fn get_pids(self) -> Vec<PidUnit> {
        self.pids
    }
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ChildStatus {
    Alive,
    Dead
}

impl Pid {
    pub fn from(pid: pid_t) -> Self {
        return Self { pid: pid };
    }
    pub fn get_raw(&self) -> pid_t {
        return self.pid;
    }
    pub fn setsid() -> Result<(), Errno> {
        let _result = unsafe { syscall!(Sysno::setsid) }?;
        return Ok(());
    }
    pub fn kill(&self) -> Result<(), LabeledError> {
        let pid_no = self.get_raw();
        const SIGTERM: u8 = 15;
        unsafe {
            syscall!(
                Sysno::kill,
                pid_no,
                SIGTERM
            )
        }.map_err(|e| {
            make_error!(
                format!("Failed to kill the process id: {}", pid_no),
                format!("Error: {e:?}"),
                nu_protocol::Span::unknown()
            )
        })?;

        Ok(())
    }
}
