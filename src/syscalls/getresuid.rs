#[allow(unused)]
///Note: This is just for abi consistency.
#[allow(non_camel_case_types)]
type uid_t = u32;
pub struct Uid {
    uid: uid_t,
}

#[derive(Debug)]
pub enum Errors {
    SyscallError(syscalls::Errno),
    PrivilegeError {
        ruid: uid_t,
        euid: uid_t,
        suid: uid_t,
    },
}

impl Uid {
    ///Create a new Uid type from a u32 number.
    pub fn from(id: uid_t) -> Self {
        Self { uid: id }
    }
    ///Get a u32(uid_t) from Uid. NOTE: THIS CONSUMES THE STRUCT.
    pub fn get_raw(self) -> uid_t {
        self.uid //Return uid.
    }

    ///Calls getresuid syscall.
    pub fn getresuid() -> Result<Self, Errors> {
        use syscalls::{syscall, Sysno};
        let mut ruid: uid_t = 0;
        let mut euid: uid_t = 0;
        let mut suid: uid_t = 0;

        let result = unsafe {
            syscall!(
                Sysno::getresuid,
                &mut ruid as *mut uid_t,
                &mut euid as *mut uid_t,
                &mut suid as *mut uid_t
            )
        };

        if let Err(error) = result {
            return Err(Errors::SyscallError(error));
        }

        if ruid == euid && euid == suid {
            Ok(Self::from(ruid))
        } else {
            println!("{}, {}, {} are not equal.", ruid, euid, suid);
            Err(Errors::PrivilegeError { ruid, euid, suid })
        }
    }
}
