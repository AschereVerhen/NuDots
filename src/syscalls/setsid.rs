use syscalls::{syscall, Errno, Sysno};

#[allow(non_camel_case_types)]
type pid_t = u32;
pub struct Pid {
    pub pid: pid_t,
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
}
