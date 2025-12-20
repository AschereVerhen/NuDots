//This is for pkg management system.
use nu_plugin::{
    EngineInterface, EvaluatedCall, PluginCommand
};
use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type
};
use crate::Nudo;
use crate::commands::utils::detectos::{OS, detect_os_raw};
pub struct Uninstall;
use crate::commands::pkg_manager::lib::{PkgOp, create_command};
pub fn uninstall(call: &EvaluatedCall, packages: Vec<String>, os: OS, no_confirm: bool, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, no_confirm, PkgOp::Uninstall)

} 

impl PluginCommand for Uninstall {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo pkg uninstall" //Installation.
    }
    fn description(&self) -> &str {
        "Allows you to uninstall packages os-agnostically"
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Package Management".to_string()))
            .rest(
                "Packages",
                SyntaxShape::String,
                "The Packages to uninstall."
            )
            .add_help()
            .switch("yes", "Skip Confirmation", Some('y'))
            .input_output_type(Type::Any, Type::Nothing) //Takes in anything; returns nothing.
            .allows_unknown_args() //Allow people to pass pkg_manager-specific flags, like --one-shot in emerge or --overwrite="*" in pacman.
    }
    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            nu_protocol::Example {
                example: "nudo pkg uninstall hyprland qs emerge",
                description: "Easily uninstall packages without having to memorize your distro's flags.",
                result: None,
            },
            nu_protocol::Example {
                example: "['waybar', 'startx', 'bluetoothctl'] | nudo pkg uninstall",
                description: "Also takes in from stdin.",
                result: None,
            }
        ]
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
        uninstall(call, packages, os, no_confirm, engine)?;
        Ok(PipelineData::Empty)
    }
}