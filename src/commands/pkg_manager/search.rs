//This is for pkg management system.
use nu_plugin::{
    EngineInterface, EvaluatedCall, PluginCommand
};
use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type
};
use crate::Nudo;
use crate::commands::utils::detectos::{OS, detect_os_raw};
pub struct Search;
use crate::commands::pkg_manager::lib::{PkgOp, create_command};
pub fn search(call: &EvaluatedCall, packages: Vec<String>, os: OS, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, false, PkgOp::Search)

} 

impl PluginCommand for Search {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nupkg search" //Installation.
    }
    fn description(&self) -> &str {
        "Allows you to search packages os-agnostically"
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Package Management".to_string()))
            .add_help()
            .input_output_type(Type::Any, Type::Nothing) //Takes in anything; returns nothing.
            .allows_unknown_args() //Allow people to pass pkg_manager-specific flags, like --one-shot in emerge or --overwrite="*" in pacman.
            .named(
                "Search Term",
                SyntaxShape::String,
                "The term to search for",
                Some('s')
            )
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
        //Firstly, lets check rest:
        let search_term: String = call.req(0)?;

        let os = detect_os_raw();
        search(call, vec![search_term], os, engine)?;
        Ok(PipelineData::Empty)
    }
}