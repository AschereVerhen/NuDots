use syscalls::{
    syscall,
    Sysno,
    Errno,
};

use crate::syscalls::kill::Pid;



pub fn setpgid(pid: Pid, pgid: Pid) -> Result<(), Errno> {
    
    let pid_int = pid.into_int();
    let pgid_int = pgid.into_int();
    
    if pgid_int < 0 {
        return Err(Errno::EINVAL);
    }
    
    unsafe {
        syscall!(
            Sysno::setpgid,
            pid_int,
            pgid_int
        )
    }?;
    Ok(())
}