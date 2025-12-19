
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
            Box::new(NudoDispatch),
            Box::new(dependency::DependencyCheck),
            Box::new(anyoneof::AnyOneOf),
            Box::new(detectos::DetectOs),
            Box::new(argsrequired::ArgsRequired),
            Box::new(run::Run),
            Box::new(install::Install),
        ]
    }
}