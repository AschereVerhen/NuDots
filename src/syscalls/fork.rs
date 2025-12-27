use crate::syscalls::setsid::{ChildStatus, Pid};
use syscalls::{syscall, Errno, Sysno};

impl Pid {
    ///Returns the pid of child.
    pub fn fork() -> Result<Self, Errno> {
        let result = unsafe { syscall!(Sysno::fork) }?;

        Ok(Pid::from(result as u32)) //Return the PID of the child process.
    }
    pub fn wait_for_child(&self) -> Result<ChildStatus, Errno> {
        let mut status: i32 = 0;
        let _result = unsafe {
            syscall!(
                Sysno::wait4,
                self.get_raw(),
                &mut status as *mut i32 as usize,
                0,
                std::ptr::null::<usize>()
            )
        }?;
        Ok(ChildStatus::Dead) //If the above; it means the child is dead.
    }
}
