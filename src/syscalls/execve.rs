use std::ffi::{c_char, CString};
use std::os::unix::ffi::OsStrExt;
use syscalls::{syscall, Errno, Sysno};

use which::which;

//We want to take ownership, actually.
pub fn execve(arguments: Vec<&CString>, env: Vec<&CString>) -> Result<(), Errno> {
    //Our version of execve only takes the Vector of arguments and append NULL to it by itself;
    // and takes in the env aswell and appends NULL to it by itself.

    let mut safe_args_ptrs: Vec<*const c_char> = arguments.iter().map(|a| a.as_ptr()).collect();
    safe_args_ptrs.push(std::ptr::null());

    let mut safe_env_ptrs: Vec<*const c_char> = env.iter().map(|a| a.as_ptr()).collect();
    safe_env_ptrs.push(std::ptr::null());

    let program = match arguments[0].to_str() {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 in arguments. (FROM: execve function.). Err: {e:?}"),
    };
    let path_to_program = match which(program) {
        Ok(s) => s,
        Err(e) => panic!("The executable: {program} could not be found in PATH. Err: {e:?}"),
    };
    let c_path = match CString::new(path_to_program.as_os_str().as_bytes()) {
        Ok(s) => s,
        Err(e) => panic!(
            "Failed to convert Path of program: {program} to CString as it contained\
         a NULL byte. Err: {e:?}"
        ),
    };

    unsafe {
        syscall!(
            Sysno::execve,
            c_path.as_ptr(),
            safe_args_ptrs.as_ptr(),
            safe_env_ptrs.as_ptr()
        )
    }?;

    Ok(())
}
