use syscalls::{
    syscall,
    Sysno,
};

pub fn getresuid() -> (u32, u32, u32) {
    let mut realuid = 0;
    let mut effectiveuid = 0;
    let mut saveduid = 0;
    unsafe {
        syscall!(
            Sysno::getresuid,
            &mut realuid as *mut u32,
            &mut effectiveuid as *mut u32,
            &mut saveduid as *mut u32
        ).unwrap();
    }
    return (realuid, effectiveuid, saveduid);
}

pub fn userisroot() -> bool {
    let (realuid, effectiveuid, saveduid) = getresuid();
    return realuid == 0 && realuid == effectiveuid && effectiveuid == saveduid;
}