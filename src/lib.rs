use nu_plugin::Plugin;
pub mod commands;
pub mod syscalls;
pub mod utils;
use crate::{
    commands::nustart::NuStart,
    commands::add::Add,
    commands::get::Get,
    commands::remove::Remove,
};

pub struct NuStartPlugin;

impl Plugin for NuStartPlugin {
    fn version(&self) -> String {
        "0.1.0".to_string()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            mybox![NuStart],
            mybox![Add],
            mybox![Get],
            mybox![Remove],
        ]
    }
}