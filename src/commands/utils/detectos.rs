use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, Value, Type
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::Nudo;

pub struct DetectOs;

#[derive(Debug)]
pub enum Distro {
    Arch,
    Gentoo,
    Debian,
    RedHat,
    Suse,
    NixOS,
    UnknownLinux,
}
pub enum OS {
    Windows, //Windonts
    MacOS, //Macos Based system
    FreeBSD,
    OpenBSD,
    DragonflyBSD, //dragonfly
    NetBSD,
    Linux(Distro),
    UnknownOS,
}

impl From<OS> for String {
    fn from(os: OS) -> Self {
        match os {
            OS::Linux(distro) => format!("{:?} Linux", distro),
            _ => std::env::consts::OS.to_string()
        }
    }
}

fn detect_distro() -> Distro {
    let deps: Vec<String> = vec!["pacman".into(), "emerge".into(), "apt".into(), "dnf".into(), "zypper".into(), "nixos-rebuild".into()];
    let returned = crate::commands::utils::anyoneof::anyoneof_raw(&deps).unwrap_or("Unknown Linux".into());
    match returned.as_str() {
        "pacman" => Distro::Arch,
        "emerge" => Distro::Gentoo,
        "apt" => Distro::Debian,
        "dnf" => Distro::RedHat,
        "zypper" => Distro::Suse,
        "nixos-rebuild" => Distro::NixOS,
        _ => Distro::UnknownLinux
    }
}

pub fn detect_os_raw() -> OS {
    let os = std::env::consts::OS;
    match os {
        "linux" => OS::Linux(detect_distro()),
        "windows" => OS::Windows,
        "macos" => OS::MacOS,
        "freebsd" => OS::FreeBSD,
        "dragonfly" => OS::DragonflyBSD,
        "openbsd" => OS::OpenBSD,
        "netbsd" => OS::NetBSD,
        _ => OS::UnknownOS,
    }
    
}

fn detect_os(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let os = detect_os_raw();
    return Ok(PipelineData::value(Value::string(os, call.head), None));
}

impl PluginCommand for DetectOs {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev detectos"
    }
    fn description(&self) -> &str {
        "This command detects and returns Your Operating System. And if its Linux Or BSD, it will also return the Distro."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .add_help()
            .input_output_type(Type::Nothing, Type::Nothing)
            .category(Category::Custom("Developer".to_string()))
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
            detect_os(call)
    }
}
