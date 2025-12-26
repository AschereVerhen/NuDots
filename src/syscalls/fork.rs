use crate::syscalls::setsid::Pid;
use syscalls::{syscall, Errno, Sysno};

impl Pid {
    ///Returns the pid of child.
    pub fn fork() -> Result<Self, Errno> {
        let result = unsafe { syscall!(Sysno::fork) }?;

        Ok(Pid::from(result as u32)) //Return the PID of the child process.
    }
}
