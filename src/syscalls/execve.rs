use std::ffi::{c_char, CString};
use std::os::unix::ffi::OsStrExt;
use nu_plugin::EvaluatedCall;
use syscalls::{syscall, Sysno};
use nu_protocol::LabeledError;
use which::which;
use crate::{make_error, return_error};

//We want to take ownership, actually.
pub fn execve(arguments: Vec<CString>, env: Vec<CString>, call: &EvaluatedCall) -> Result<(), LabeledError> {
    //Our version of execve only takes the Vector of arguments and append NULL to it by itself;
    // and takes in the env aswell and appends NULL to it by itself.

    let mut safe_args_ptrs: Vec<*const c_char> = arguments.iter().map(|a| a.as_ptr()).collect();
    safe_args_ptrs.push(std::ptr::null());

    let mut safe_env_ptrs: Vec<*const c_char> = env.iter().map(|a| a.as_ptr()).collect();
    safe_env_ptrs.push(std::ptr::null());

    let program = match arguments[0].to_str() {
        Ok(s) => s,
        Err(_e) => return_error!(
            format!("The name in command: {:?} had invalid utf-8 characters.", arguments[0]),
            "Please provide a valid name.",
            call.head
        )
    };
    let path_to_program = match which(program) {
        Ok(s) => s,
        Err(_e) => return_error!(
            format!("The program: {} Was not found in the path.", program),
            "Please install the program or provide the correct name.",
            call.head
        )
    };
    let c_path = match CString::new(path_to_program.as_os_str().as_bytes()) {
        Ok(s) => s,
        Err(e) => return_error!(
            format!("Failed to convert Path of program: {program} to CString as it contained a NULL byte. Err: {e:?}"),
            "Please ensure the path of the program contains no Null Bytes(\\0).",
            call.head
        ),
    };

    let _result = unsafe {
        syscall!(
            Sysno::execve,
            c_path.as_ptr(),
            safe_args_ptrs.as_ptr(),
            safe_env_ptrs.as_ptr()
        )
    }.map_err(|e| {
        make_error!(
            format!("Failed to execve {:?}: {:?}", path_to_program, e),
            "Execve",
            call.head
        )
    })?;

    Ok(())
}
