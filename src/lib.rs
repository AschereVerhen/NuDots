
pub mod commands;
use nu_plugin::Plugin;
use commands::{
    nudo::NudoDispatch,
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
            Box::new(DependencyCheck),
            Box::new(AnyOneOf),
            Box::new(DetectOs),
            Box::new(ArgsRequired),
            Box::new(Run),
        ]
    }
}