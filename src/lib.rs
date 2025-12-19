
pub mod commands;
pub mod errors;
use nu_plugin::Plugin;
use commands::{
    nudo::NudoDispatch,
    pkg_manager::*,
    utils::*,
};

pub struct Nudo;

impl Plugin for Nudo {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(NudoDispatch), //main nudo
            Box::new(dev::Dev), //nudo dev
            Box::new(dependency::DependencyCheck), //nudo dev dependcheck
            Box::new(anyoneof::AnyOneOf), //nudo dev anyoneof
            Box::new(detectos::DetectOs), //nudo dev detectos
            Box::new(argsrequired::ArgsRequired), //nudo dev argsrequired
            Box::new(run::Run), //nudo dev run
            Box::new(pkg::Pkg), //nudo pkg
            Box::new(install::Install),
        ]
    }
}