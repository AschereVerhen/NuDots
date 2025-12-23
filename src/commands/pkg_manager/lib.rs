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

macro_rules! manager_to_str {
    ($s: expr) => {
        match $s {
            Manager::Paru => "paru",
            Manager::Yay => "yay",
            Manager::Pacman => "pacman",
            Manager::Emerge => "emerge",
            Manager::Zypper => "zypper",
            Manager::Dnf => "dnf",
            Manager::Apt => "apt",
            Manager::Unknown => "unknown",
        }
    };
}

impl Manager {
    pub fn req_sudo(&self) -> bool {
        !matches!(self, Manager::Paru | Manager::Yay)
    }
    pub fn as_str(&self) -> &'static str {
        // match self {
        //     Manager::Paru => "paru",
        //     Manager::Yay => "yay",
        //     Manager::Pacman => "pacman",
        //     Manager::Emerge => "emerge",
        //     Manager::Zypper => "zypper",
        //     Manager::Dnf => "dnf",
        //     Manager::Apt => "apt",
        //     Manager::Unknown => "unknown",
        // }
        manager_to_str!(self)
    }
}

use std::fmt;
impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let s = match self {
        //     Manager::Paru => "paru",
        //     Manager::Yay => "yay",
        //     Manager::Pacman => "pacman",
        //     Manager::Emerge => "emerge",
        //     Manager::Zypper => "zypper",
        //     Manager::Dnf => "dnf",
        //     Manager::Apt => "apt",
        //     Manager::Unknown => "unknown",
        // };
        let s = manager_to_str!(self);
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

impl OpSpec {
    fn new(command: &'static str, args: &'static [&'static str], nc_arg: &'static [&'static str], needs_root: bool) -> Self {
        Self {
            command: command,
            args: args,
            nc_arg: nc_arg,
            needs_root: needs_root,
        }
    }
}
impl Manager {
    pub fn op_spec(&self, op: PkgOp) -> Option<OpSpec> {
        use Manager::*;
        use PkgOp::*;

        Some(match (self, op) {
            // ============================================================
            // ARCH LINUX
            // ============================================================
            (Paru | Pacman | Yay, Install)        => OpSpec::new(self.as_str(), &["-S"],   &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, Uninstall)      => OpSpec::new(self.as_str(), &["-Rns"], &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, Update)         => OpSpec::new(self.as_str(), &["-Syu"], &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, Search)         => OpSpec::new(self.as_str(), &["-Ss"],  &[],              self.req_sudo()),
            (Paru | Pacman | Yay, CleanCache)     => OpSpec::new(self.as_str(), &["-Scc"],  &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, ListInstalled)  => OpSpec::new(self.as_str(), &["-Q"],   &[],              self.req_sudo()),

            // ============================================================
            // GENTOO
            // ============================================================
            (Emerge, Install)      => OpSpec::new("emerge", &["--ask"],                &["--quiet"], true),
            (Emerge, Uninstall)    => OpSpec::new("emerge", &["--unmerge", "--ask"],   &["--quiet"], true),
            (Emerge, Update)       => OpSpec::new("emerge", &["-avuDN", "@world"],     &["--quiet"], true),
            (Emerge, Search)       => OpSpec::new("emerge", &["--search"],             &[],          false),
            (Emerge, CleanCache)   => OpSpec::new("emerge", &["--depclean"],           &["--quiet"], true),
            (Emerge, ListInstalled)=> OpSpec::new("qlist",  &["-I"],                   &[],          false),

            // ============================================================
            // RPM FAMILY
            // ============================================================
            (Dnf, Install)         => OpSpec::new("dnf", &["install"],           &["-y"], true),
            (Dnf, Uninstall)       => OpSpec::new("dnf", &["remove"],            &["-y"], true),
            (Dnf, Update)          => OpSpec::new("dnf", &["upgrade"],           &["-y"], true),
            (Dnf, Search)          => OpSpec::new("dnf", &["search"],            &[],     false),
            (Dnf, CleanCache)      => OpSpec::new("dnf", &["clean", "all"],      &["--yes"], true),
            (Dnf, ListInstalled)   => OpSpec::new("dnf", &["list", "--installed"], &[],   false),

            (Zypper, Install)      => OpSpec::new("zypper", &["install"],               &["-y"], true),
            (Zypper, Uninstall)    => OpSpec::new("zypper", &["remove"],                &["-y"], true),
            (Zypper, Update)       => OpSpec::new("zypper", &["update"],                &["-y"], true),
            (Zypper, Search)       => OpSpec::new("zypper", &["search"],                &[],     false),
            (Zypper, CleanCache)   => OpSpec::new("zypper", &["clean"],                 &["-y"], true),
            (Zypper, ListInstalled)=> OpSpec::new("zypper", &["packages", "--installed-only"], &[], false),

            // ============================================================
            // DEBIAN FAMILY
            // ============================================================
            (Apt, Install)         => OpSpec::new("apt", &["install"],           &["-y"], true),
            (Apt, Uninstall)       => OpSpec::new("apt", &["remove"],            &["-y"], true),
            (Apt, Update)          => OpSpec::new("apt", &["update"],            &["-y"], true),
            (Apt, Search)          => OpSpec::new("apt", &["search"],            &[],     false),
            (Apt, CleanCache)      => OpSpec::new("apt", &["clean"],             &["-y"], true),
            (Apt, ListInstalled)   => OpSpec::new("apt", &["list", "--installed"], &[],   false),

            (Unknown, _) => return None,
        })
    }
}