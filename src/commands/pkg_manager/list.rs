//This is for pkg management system.
use nu_plugin::{
    EngineInterface, EvaluatedCall, PluginCommand
};
use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, Type
};
use crate::Nudo;
use crate::commands::utils::detectos::{OS, detect_os_raw};
pub struct ListPkg;
use crate::commands::pkg_manager::lib::{PkgOp, create_command};
pub fn list(call: &EvaluatedCall, packages: Vec<String>, os: OS, no_confirm: bool, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, no_confirm, PkgOp::ListInstalled)
} 

impl PluginCommand for ListPkg {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nupkg list" //Installation.
    }
    fn description(&self) -> &str {
        "Allows you to list all the packages installed on your system os-agnostically"
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Package Management".to_string()))
            .add_help()
            .input_output_type(Type::Nothing, Type::Nothing) //Takes in anything; returns nothing.
            .allows_unknown_args() //Allow people to pass pkg_manager-specific flags, like --one-shot in emerge or --overwrite="*" in pacman.
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
        //Firstly, lets check rest:
        let mut packages: Vec<String> = call.rest(0)?;
        let mut packages_stdin: Vec<String> = Vec::new();
        for value in input {
            packages_stdin.push(value.as_str()?.to_string())
        }
        //We take in both from args AND Stdin.
        packages.extend(packages_stdin); //Now we will not use packages_stdin.
        let no_confirm:bool  = call.has_flag("yes")?;
        let os = detect_os_raw();
        list(call, packages, os, no_confirm, engine)?;
        Ok(PipelineData::Empty)
    }
}