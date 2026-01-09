pub use nu_plugin::{
    EngineInterface,
    EvaluatedCall,
    PluginCommand
};
pub use nu_protocol::{
    Category,
    LabeledError,
    PipelineData,
    Signature,
    SyntaxShape,
    Type,
    Example
};
pub use plugincmd_procmacro::plugin_command;
pub use crate::Nudo;
pub use crate::commands::utils::detectos::{
    OS,
    detect_os_raw
};
pub use crate::commands::pkg_manager::lib::{
    PkgOp,
    create_command
};