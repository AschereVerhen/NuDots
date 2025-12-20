use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, Value, Type
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::{Nudo, commands::pkg_manager::lib::{Manager, detect_archpkg}};

pub struct DetectOs;

#[derive(Debug)]
pub enum Distro {
    Arch,
    Gentoo,
    Debian,
    RedHat,
    Suse,
    UnknownLinux,
}
pub enum OS {
    Linux(Distro),
    UnknownOS,
}

impl From<OS> for String {
    fn from(os: OS) -> Self {
        match os {
            OS::Linux(distro) => format!("{:?} Linux", distro),
            _ => "unknown".to_string()
        }
    }
}

impl OS {
    pub fn which_manager(&self) -> Manager {
        match self {
            OS::Linux(distro) => {
                match distro {
                    Distro::Arch => {
                        let detected = detect_archpkg();
                        match detected {
                            Ok("paru") => Manager::Paru,
                            Ok("yay") => Manager::Yay,
                            Ok("pacman") => Manager::Pacman,
                            _ => Manager::Unknown,
                        }
                    },
                    Distro::Gentoo => Manager::Emerge,
                    Distro::Suse => Manager::Zypper,
                    Distro::RedHat => Manager::Dnf,
                    Distro::Debian => Manager::Apt,
                    Distro::UnknownLinux => Manager::Unknown
                }
            },
            _ => Manager::Unknown
        }
    }
}


fn detect_distro() -> Distro {
    let deps: Vec<&str> = vec!["pacman", "emerge", "apt", "dnf", "zypper"];
    let returned = crate::commands::utils::anyoneof::anyoneof_raw(&deps).unwrap_or(&"Unknown Linux");
    match *returned {
        "pacman" => Distro::Arch,
        "emerge" => Distro::Gentoo,
        "apt" => Distro::Debian,
        "dnf" => Distro::RedHat,
        "zypper" => Distro::Suse,
        _ => Distro::UnknownLinux
    }
}

pub fn detect_os_raw() -> OS {
    let os = std::env::consts::OS;
    match os {
        "linux" => OS::Linux(detect_distro()),
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
        "This subcommand detects and returns Your Operating System. And if its Linux Or BSD, it will also return the Distro."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .add_help()
            .input_output_type(Type::Nothing, Type::Nothing)
            .category(Category::Custom("Developer".to_string()))
    }
    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![nu_protocol::Example {
            example: "nudo dev detectos",
            description: "Easily Know your distro or OS",
            result: Some(Value::test_string("Arch Linux"))
        }]
    }
    fn run(
            &self,
            _plugin: &Nudo,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
            detect_os(call)
    }
}
