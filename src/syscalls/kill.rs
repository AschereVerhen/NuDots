use syscalls::{
    syscall,
    Sysno,
    Errno
};
#[allow(nonstandard_style)]
pub type pid_t = i32;
#[derive(PartialEq)] //No Need for anything else. 
pub struct Pid(pid_t); //Pid type
#[repr(i32)]
#[allow(dead_code)]
pub enum Signals {
    SIGTERM = 15,
    SIGINT = 2,
    SIGKILL = 9,
    SIGSTOP = 19,
    INVALID = -1, //Should not happen.
}
#[allow(dead_code)]
impl Signals {
    pub fn from_raw(sig: i32) -> Self {
        match sig {
            2 => Self::SIGINT,
            9 => Self::SIGKILL,
            15 => Self::SIGTERM,
            19 => Self::SIGSTOP,
            _ => Self::INVALID,
        }
    }
    pub fn to_int(&self) -> pid_t {
        match self {
            Self::SIGINT => 2,
            Self::SIGKILL => 9,
            Self::SIGTERM => 15,
            Self::SIGSTOP => 19,
            Self::INVALID => -1,
        }
    }
}
impl Pid {
    pub fn from_raw(pid_t: i32) -> Self {
        return Pid(pid_t)
    }
    pub fn into_int(&self) -> i32 {
        self.0
    }
}

impl std::fmt::Display for Pid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
/// Kills a singular process.
#[allow(dead_code)]
pub fn kill(pid: Pid, sig: Signals) -> Result<(), Errno> {
    let signal = sig.to_int();
    let pid_int = pid.0;
    if signal == -1 {
        return Err(Errno::EINVAL);
    }

    unsafe {
        syscall!(
            Sysno::kill,
            pid_int,
            signal
        )
    }?;

    Ok(())
}
/// Kills a whole process group.
pub fn killpg(pid: Pid, sig: Signals) -> Result<(), Errno> {
    let signal = sig.to_int();
    let mut pid_int = pid.0;

    if pid_int > 0 {
        pid_int *= -1
    }

    if signal == -1 {
        return Err(Errno::EINVAL);
    }

    unsafe {
        syscall!(
            Sysno::kill,
            pid_int,
            signal
        )
    }?;

    Ok(())
}