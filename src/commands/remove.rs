use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Example, LabeledError, PipelineData, Signature, SyntaxShape, Type};
use crate::utils::save::{ConfigFile, ConfigUnit};
use crate::utils::writelogic::{get_config, write_configfile};
use crate::NuStartPlugin;

pub struct Remove;


pub fn remove(remove_cmd: Option<String>, index: Option<u32>, everything: bool) -> Result<PipelineData, LabeledError> {
    let configfile = get_config();
    let configfile = configfile.get_programs();
    //if its everything, we just nuke the whole config file and write only the basic struct.
    if everything {
        let new_config_file = ConfigFile::new(vec![]);
        write_configfile(new_config_file).map_err(|e| LabeledError::new(e.to_string()))?;
        return Ok(PipelineData::Empty)
    }
    if index.is_some_and(|x| x < configfile.len() as u32) {
        let index = index.unwrap();
        let mut new_configunits: Vec<ConfigUnit> = Vec::new();
        let mut counter = 0;
        loop {
            if counter == configfile.len() { break } //prevent indexing over vector's bounds.
            if counter != index as usize {
                new_configunits.push(configfile[counter].clone());
                counter += 1;
            } else {
                counter += 1;
                continue;
            }
        }
        let new_config_file = ConfigFile::new(new_configunits);
        write_configfile(new_config_file).map_err(|e| LabeledError::new(e.to_string()))?;
        return Ok(PipelineData::Empty)
    } else if index.is_some_and(|x| x > configfile.len() as u32) { return Err(LabeledError::new("The index you entered does not exist!")) }
    if let Some(target_cmd) = remove_cmd {
        let mut new_confunits: Vec<ConfigUnit> = Vec::new();
        let iter_vec = configfile.iter();
        for confunit in iter_vec {
            if confunit.get_name() == target_cmd {
                continue;
            }
            new_confunits.push(confunit.clone());
        }
        let new_config_file = ConfigFile::new(new_confunits);
        write_configfile(new_config_file).map_err(|e| LabeledError::new(e.to_string()))?;
        return Ok(PipelineData::Empty)
    }

    // config unit to the autostart database.
    Ok(PipelineData::Empty)
}

impl PluginCommand for Remove {
    type Plugin = NuStartPlugin;

    fn name(&self) -> &str {
        "nustart remove"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("Command", SyntaxShape::String, "Command to add")
            .switch(
                "all",
                "Choose to remove all the programs. None will remain.",
                Some('a'),
            )
            .named(
                "index",
                SyntaxShape::Int,
                "Select the exact index of the table to remove",
                Some('i'),
            )
            .add_help()
            .input_output_types(vec![
                (Type::Nothing, Type::Nothing),
            ])
    }

    fn description(&self) -> &str {
        "NuStart Remove: Remove a command from autostart database."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["enable", "save", "add"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let program: Option<String> = call.opt(0)?;
        let everything = call.has_flag("all")?;
        let index = call.get_flag::<u32>("index")?;
        remove(program, index, everything)?;
        Ok(PipelineData::Empty)
    }
}
