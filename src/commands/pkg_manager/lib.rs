use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::LabeledError;

use crate::{
    commands::utils::{anyoneof::anyoneof_raw, detectos},
    errors::MyError,
};
pub fn detect_priv<'a>() -> Result<&'a str, MyError> {
    let options: Vec<&str> = vec!["sudo", "doas", "run0"];
    let priv_exec = anyoneof_raw(&options)?;
    Ok(priv_exec)
}

pub fn detect_archpkg<'a>() -> Result<&'a str, MyError> {
    let options: Vec<&str> = vec!["paru", "yay", "pacman"];
    let detected = anyoneof_raw(&options)?;
    Ok(detected)
}


pub fn create_command(
    call: &EvaluatedCall,
    engine: &EngineInterface,
    packages: Vec<String>,
    os: detectos::OS,
    no_confirm: bool,
    mode: PkgOp,
) -> Result<(), LabeledError> {
    let manager: Manager 
    = match os.which_manager() {
            val if val == Manager::Unknown => {
                return Err(
                    LabeledError::new("Could not detect which PKG Manager you have installed on your system.")
                        .with_label("This returned Unknown; note that if your pkg manager is not supported this might happen.", call.head)
                        .with_help("Maybe try installing one for your os?")
                )
            },
            val if val != Manager::Unknown => {
                val
            },
            _ => Manager::Unknown, //This route will not come. Just here to make the compiler shut tf up.
    };

    let opspec: OpSpec 
    = match manager.op_spec(mode) {
        Some(op) => op,
        Option::None => {
            return Err(LabeledError::new(
                "Could not detect which PKG Manager you have installed on your system.",
            )
            .with_label("This returned Unknown", call.head)
            .with_help("Maybe try installing one for your os?"));
        }
    };
    let mut pkg_args: Vec<String> = opspec.args.iter().map(|element| element.to_string()).collect::<Vec<String>>();
    if no_confirm {
        pkg_args.extend(opspec.nc_arg.iter().map(|element| element.to_string()).collect::<Vec<String>>());
    }
    use crate::syscalls::getresuid::userisroot;
    let command: String; 
    if ! userisroot() && ! manager.req_sudo() {
        let priv_exec = detect_priv();
        if let Ok(priv_exe) = priv_exec {
            command = priv_exe.to_string();
            pkg_args.insert(0, manager.to_string());
        } else {
            return Err(LabeledError::new(
                "Could not find any priviledge escalatory Tools in your system.")
            .with_label("Dependency Check failed", call.head)
            .with_help("Maybe install either one of sudo, doas, run0?"));
        }
    } else {
        command = manager.to_string();
    }
    let mut pkg_args = pkg_args.iter().map(|element| element.to_string()).collect::<Vec<String>>();
    pkg_args.extend(packages);
    //Now we use run:
    use crate::commands::utils::run::run;
    run(call,
        command,
        pkg_args,
        engine
    )?;
    Ok(())
}



#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Manager {
    Paru,
    Yay,
    Pacman,
    Emerge,
    Zypper,
    Dnf,
    Apt,
    Unknown,
}

impl Manager {
    pub fn req_sudo(&self) -> bool {
        !matches!(self, Manager::Paru | Manager::Yay)
    }
}

use std::fmt;
impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Manager::Paru => "paru",
            Manager::Yay => "yay",
            Manager::Pacman => "pacman",
            Manager::Emerge => "emerge",
            Manager::Zypper => "zypper",
            Manager::Dnf => "dnf",
            // Manager::Nix => "nix",
            Manager::Apt => "apt",
            Manager::Unknown => "unknown",
        };
        write!(f, "{s}")
    }
}
#[derive(Debug, Clone, Copy)]
pub enum PkgOp {
    Install,
    Uninstall,
    Update,
    Search,
    CleanCache,
    ListInstalled,
}

//Be Aware, from now on, repetition is the only god. Do not read the below unless you **Enjoy** pain.
pub struct OpSpec {
    pub command: &'static str,
    pub args: &'static [&'static str],
    pub nc_arg: &'static [&'static str],
    pub needs_root: bool,
}
impl Manager {
    pub fn op_spec(&self, op: PkgOp) -> Option<OpSpec> {
        use Manager::*;
        use PkgOp::*;

        Some(match (self, op) {
            // ============================================================
            // ARCH LINUX(Checked. Working.)
            // ============================================================
            (Paru, Install) => OpSpec {
                command: "paru",
                args: &["-S"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Paru, Uninstall) => OpSpec {
                command: "paru",
                args: &["-Rns"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Paru, Update) => OpSpec {
                command: "paru",
                args: &["-Syu"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Paru, Search) => OpSpec {
                command: "paru",
                args: &["-Ss"],
                nc_arg: &[],
                needs_root: false,
            },
            (Paru, CleanCache) => OpSpec {
                command: "paru",
                args: &["-Sc"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Paru, ListInstalled) => OpSpec {
                command: "paru",
                args: &["-Q"],
                nc_arg: &[],
                needs_root: false,
            },

            (Yay, Install) => OpSpec {
                command: "yay",
                args: &["-S"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Yay, Uninstall) => OpSpec {
                command: "yay",
                args: &["-Rns"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Yay, Update) => OpSpec {
                command: "yay",
                args: &["-Syu"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Yay, Search) => OpSpec {
                command: "yay",
                args: &["-Ss"],
                nc_arg: &[],
                needs_root: false,
            },
            (Yay, CleanCache) => OpSpec {
                command: "yay",
                args: &["-Sc"],
                nc_arg: &["--noconfirm"],
                needs_root: false,
            },
            (Yay, ListInstalled) => OpSpec {
                command: "yay",
                args: &["-Q"],
                nc_arg: &[],
                needs_root: false,
            },

            (Pacman, Install) => OpSpec {
                command: "pacman",
                args: &["-S"],
                nc_arg: &["--noconfirm"],
                needs_root: true,
            },
            (Pacman, Uninstall) => OpSpec {
                command: "pacman",
                args: &["-Rns"],
                nc_arg: &["--noconfirm"],
                needs_root: true,
            },
            (Pacman, Update) => OpSpec {
                command: "pacman",
                args: &["-Syu"],
                nc_arg: &["--noconfirm"],
                needs_root: true,
            },
            (Pacman, Search) => OpSpec {
                command: "pacman",
                args: &["-Ss"],
                nc_arg: &[],
                needs_root: false,
            },
            (Pacman, CleanCache) => OpSpec {
                command: "pacman",
                args: &["-Sc"],
                nc_arg: &["--noconfirm"],
                needs_root: true,
            },
            (Pacman, ListInstalled) => OpSpec {
                command: "pacman",
                args: &["-Q"],
                nc_arg: &[],
                needs_root: false,
            },

            // ============================================================
            // GENTOO
            // ============================================================
            (Emerge, Install) => OpSpec {
                command: "emerge",
                args: &["--ask"],
                nc_arg: &["--quiet"],
                needs_root: true,
            },
            (Emerge, Uninstall) => OpSpec {
                command: "emerge",
                args: &["--unmerge", "--ask"],
                nc_arg: &["--quiet"],
                needs_root: true,
            },
            (Emerge, Update) => OpSpec {
                command: "emerge",
                args: &["-avuDN", "@world"],
                nc_arg: &["--quiet"],
                needs_root: true,
            },
            (Emerge, Search) => OpSpec {
                command: "emerge",
                args: &["--search"],
                nc_arg: &[],
                needs_root: false,
            },
            (Emerge, CleanCache) => OpSpec {
                command: "emerge",
                args: &["--depclean"],
                nc_arg: &["--quiet"],
                needs_root: true,
            },
            (Emerge, ListInstalled) => OpSpec {
                command: "qlist",
                args: &["-I"],
                nc_arg: &[],
                needs_root: false,
            },

            // ============================================================
            // RPM FAMILY(Checked; Working.)
            // ============================================================
            (Dnf, Install) => OpSpec {
                command: "dnf",
                args: &["install"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Dnf, Uninstall) => OpSpec {
                command: "dnf",
                args: &["remove"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Dnf, Update) => OpSpec {
                command: "dnf",
                args: &["upgrade"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Dnf, Search) => OpSpec {
                command: "dnf",
                args: &["search"],
                nc_arg: &[],
                needs_root: false,
            },
            (Dnf, CleanCache) => OpSpec {
                command: "dnf",
                args: &["clean", "all"],
                nc_arg: &["--yes"],
                needs_root: true,
            },
            (Dnf, ListInstalled) => OpSpec {
                command: "dnf",
                args: &["list", "--installed"],
                nc_arg: &[],
                needs_root: false,
            },

            (Zypper, Install) => OpSpec {
                command: "zypper",
                args: &["install"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Zypper, Uninstall) => OpSpec {
                command: "zypper",
                args: &["remove"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Zypper, Update) => OpSpec {
                command: "zypper",
                args: &["update"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Zypper, Search) => OpSpec {
                command: "zypper",
                args: &["search"],
                nc_arg: &[],
                needs_root: false,
            },
            (Zypper, CleanCache) => OpSpec {
                command: "zypper",
                args: &["clean"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Zypper, ListInstalled) => OpSpec {
                command: "zypper",
                args: &["packages", "--installed-only"],
                nc_arg: &[],
                needs_root: false,
            },
            (Apt, Install) => OpSpec {
                command: "apt",
                args: &["install"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Apt, Uninstall) => OpSpec {
                command: "apt",
                args: &["remove"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Apt, Update) => OpSpec {
                command: "apt",
                args: &["update"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Apt, Search) => OpSpec {
                command: "apt",
                args: &["search"],
                nc_arg: &[],
                needs_root: false,
            },
            (Apt, CleanCache) => OpSpec {
                command: "apt",
                args: &["clean"],
                nc_arg: &["-y"],
                needs_root: true,
            },
            (Apt, ListInstalled) => OpSpec {
                command: "apt",
                args: &["list", "--installed"],
                nc_arg: &[],
                needs_root: false,
            },
            (Unknown, _) => return None,
        })
    }
}
