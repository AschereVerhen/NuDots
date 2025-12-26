use syscalls::{syscall, Errno, Sysno};

#[allow(non_camel_case_types)]
type pid_t = u32;
pub struct Pid {
    pub pid: pid_t,
}

impl Pid {
    pub fn from(pid: pid_t) -> Self {
        return Self { pid: pid };
    }
    pub fn get_raw(&self) -> pid_t {
        return self.pid;
    }
    pub fn setsid() -> Result<(), Errno> {
        let result = unsafe { syscall!(Sysno::setsid) };
        if let Err(error) = result {
            return Err(error);
        }
        return Ok(());
    }
}
