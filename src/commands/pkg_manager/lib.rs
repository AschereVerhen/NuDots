use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::LabeledError;

use crate::{
    commands::utils::{anyoneof::anyoneof_raw, detectos},
    errors::MyError,
};

use crate::debugf;

use crate::syscalls::getresuid::userisroot;

pub fn detect_priv<'a>() -> Result<&'a str, MyError> {
    debugf!(detect_priv, "start");

    let options: Vec<&str> = vec!["sudo", "doas", "run0"];
    debugf!(detect_priv, "candidate privilege executors = {:?}", &options);

    let priv_exec = anyoneof_raw(&options);
    match &priv_exec {
        Ok(v) => debugf!(detect_priv, "detected privilege executor = {}", v),
        Err(e) => debugf!(detect_priv, "failed to detect privilege executor: {:?}", e),
    }
    let priv_exec = priv_exec?;

    Ok(priv_exec)
}

pub fn detect_archpkg<'a>() -> Result<&'a str, MyError> {
    debugf!(detect_archpkg, "start");

    let options: Vec<&str> = vec!["paru", "yay", "pacman"];
    debugf!(detect_archpkg, "candidate package managers = {:?}", options);

    let detected = anyoneof_raw(&options);
    match &detected {
        Ok(v) => debugf!(detect_archpkg, "detected package manager = {}", v),
        Err(e) => debugf!(detect_archpkg, "failed to detect package manager: {:?}", e),
    }
    let detected = detected?;

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
    debugf!(create_command, "start");
    debugf!(
        create_command,
        "inputs => packages={:?}, os={:?}, no_confirm={}, mode={:?}",
        packages,
        os,
        no_confirm,
        mode
    );

    let manager: Manager = match os.which_manager() {
        val if val == Manager::Unknown => {
            debugf!(create_command, "manager detection returned Unknown");
            return Err(
                LabeledError::new(
                    "Could not detect which PKG Manager you have installed on your system.",
                )
                    .with_label(
                        "This returned Unknown; note that if your pkg manager is not supported this might happen.",
                        call.head,
                    )
                    .with_help("Maybe try installing one for your os?"),
            );
        }
        val => {
            debugf!(create_command, "detected manager = {}", val);
            val
        }
    };

    let opspec: OpSpec = match manager.op_spec(mode) {
        Some(op) => {
            debugf!(
                create_command,
                "OpSpec resolved => command={}, args={:?}, nc_arg={:?}, needs_root={}",
                op.command,
                op.args,
                op.nc_arg,
                op.needs_root
            );
            op
        }
        None => {
            debugf!(
                create_command,
                "OpSpec resolution failed for manager={} mode={:?}",
                manager,
                mode
            );
            return Err(
                LabeledError::new(
                    "Could not detect which PKG Manager you have installed on your system.",
                )
                    .with_label("This returned Unknown", call.head)
                    .with_help("Maybe try installing one for your os?"),
            );
        }
    };

    let mut pkg_args: Vec<String> =
        opspec.args.iter().map(|e| e.to_string()).collect();
    debugf!(create_command, "base pkg_args = {:?}", pkg_args);

    if no_confirm {
        debugf!(create_command, "no_confirm enabled, extending args");
        pkg_args.extend(opspec.nc_arg.iter().map(|e| e.to_string()));
    }

    debugf!(create_command, "pkg_args after no_confirm = {:?}", pkg_args);

    let is_root = userisroot();
    debugf!(create_command, "userisroot() = {}", is_root);
    debugf!(create_command, "manager.req_sudo() = {}", manager.req_sudo());

    let command: String;

    if !is_root || manager.req_sudo() {
        debugf!(create_command, "privilege escalation required");

        let priv_exec = detect_priv();
        match priv_exec {
            Ok(priv_exe) => {
                debugf!(
                    create_command, 
                    "using privilege executor = {}",
                    priv_exe
                );
                command = priv_exe.to_string();
                pkg_args.insert(0, manager.to_string());
            }
            Err(e) => {
                debugf!(
                    create_command, 
                    "privilege escalation tool detection failed: {:?}",
                    e
                );
                return Err(
                    LabeledError::new(
                        "Could not find any privilege escalatory Tools in your system.",
                    )
                        .with_label("Dependency Check failed", call.head)
                        .with_help("Maybe install either one of sudo, doas, run0?"),
                );
            }
        }
    } else {
        debugf!(create_command, "privilege escalation not required");
        command = manager.to_string();
    }

    let mut pkg_args =
        pkg_args.iter().map(|e| e.to_string()).collect::<Vec<String>>();
    pkg_args.extend(packages);

    debugf!(
        create_command,
        "final command execution => command={}, args={:?}",
        command,
        pkg_args
    );

    use crate::commands::utils::run::run;

    let run_result = run(call, command, pkg_args, engine);
    match &run_result {
        Ok(_) => debugf!(create_command, "run() completed successfully"),
        Err(e) => debugf!(create_command, "run() failed: {:?}", e),
    }

    run_result?;
    Ok(())
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
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
        let needs = !matches!(self, Manager::Paru | Manager::Yay);
        debugf!(Manager_req_sudo, "manager={} => {}", self, needs);
        needs
    }

    pub fn as_str(&self) -> &'static str {
        let s = manager_to_str!(self);
        debugf!(Manager_as_str, "manager={:?} => {}", self, s);
        s
    }
}

use std::fmt;

impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub struct OpSpec {
    pub command: &'static str,
    pub args: &'static [&'static str],
    pub nc_arg: &'static [&'static str],
    pub needs_root: bool,
}

impl OpSpec {
    fn new(
        command: &'static str,
        args: &'static [&'static str],
        nc_arg: &'static [&'static str],
        needs_root: bool,
    ) -> Self {
        debugf!(
            OpSpec_new, 
            "command={}, args={:?}, nc_arg={:?}, needs_root={}",
            command,
            args,
            nc_arg,
            needs_root
        );
        Self {
            command,
            args,
            nc_arg,
            needs_root,
        }
    }
}

impl Manager {
    pub fn op_spec(&self, op: PkgOp) -> Option<OpSpec> {
        debugf!(
            Manager_op_spec, 
            "resolving op spec for manager={} op={:?}",
            self,
            op
        );

        use Manager::*;
        use PkgOp::*;

        Some(match (self, op) {
            (Paru | Pacman | Yay, Install)        => OpSpec::new(self.as_str(), &["-S"],   &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, Uninstall)      => OpSpec::new(self.as_str(), &["-Rns"], &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, Update)         => OpSpec::new(self.as_str(), &["-Syu"], &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, Search)         => OpSpec::new(self.as_str(), &["-Ss"],  &[],              self.req_sudo()),
            (Paru | Pacman | Yay, CleanCache)     => OpSpec::new(self.as_str(), &["-Scc"], &["--noconfirm"], self.req_sudo()),
            (Paru | Pacman | Yay, ListInstalled)  => OpSpec::new(self.as_str(), &["-Q"],   &[],              self.req_sudo()),

            (Emerge, Install)      => OpSpec::new("emerge", &["--ask"],                &["--quiet"], true),
            (Emerge, Uninstall)    => OpSpec::new("emerge", &["--unmerge", "--ask"],   &["--quiet"], true),
            (Emerge, Update)       => OpSpec::new("emerge", &["-avuDN", "@world"],     &["--quiet"], true),
            (Emerge, Search)       => OpSpec::new("emerge", &["--search"],             &[],          false),
            (Emerge, CleanCache)   => OpSpec::new("emerge", &["--depclean"],           &["--quiet"], true),
            (Emerge, ListInstalled)=> OpSpec::new("qlist",  &["-I"],                   &[],          false),

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

            (Apt, Install)         => OpSpec::new("apt", &["install"],           &["-y"], true),
            (Apt, Uninstall)       => OpSpec::new("apt", &["remove"],            &["-y"], true),
            (Apt, Update)          => OpSpec::new("apt", &["update"],            &["-y"], true),
            (Apt, Search)          => OpSpec::new("apt", &["search"],            &[],     false),
            (Apt, CleanCache)      => OpSpec::new("apt", &["clean"],             &["-y"], true),
            (Apt, ListInstalled)   => OpSpec::new("apt", &["list", "--installed"], &[],   false),

            (Unknown, _) => {
                debugf!(
                    Manager_op_spec,
                    "Unknown manager encountered for op={:?}",
                    op
                );
                return None;
            }
        })
    }
}
