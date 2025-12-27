use syscalls::{syscall, Errno, Sysno};

#[allow(non_camel_case_types)]
type pid_t = u32;
pub struct Pid {
    pub pid: pid_t,
}
#[derive(PartialEq)]
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
        let result = unsafe { syscall!(Sysno::setsid) }?;
        return Ok(());
    }
    ///This function probes the child process to check if its alive or ded
    pub fn probe(&self) -> Result<ChildStatus, Errno> {
        let pid = self.get_raw();
        let result = unsafe {
            syscall!(
                Sysno::kill,
                pid,
                0 //just probe.
            )
        };
        if result.err() == Some(Errno::ESRCH) {return Ok(ChildStatus::Dead);}
        let _ = result?; //Propogate the error if any.

        Ok(ChildStatus::Alive)
    }
}
