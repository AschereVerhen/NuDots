use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Value
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::Nudo;
//Firstly, implimenting the dependency_check program.
//We do not want to add this to any nu api. Except error making.

//firstly, get path variable
fn get_path_resolved(call: &EvaluatedCall) -> Result<Vec<String>, LabeledError>{
    let path = if let Some(value) = std::env::var_os("PATH") {
        value
    } else {
        //We throw an error.
        return Err(
            LabeledError::new("Could not find path")
                .with_label("$env.PATH does not seem to be set", call.head)
        );
    };
    let path = match path.into_string() {
        Ok(val) => val,
        Err(e) => {
            return Err(
                LabeledError::new("Invalid PATH")
                    .with_label(e.to_string_lossy(), call.head) //We will not use to_string_lossy since it is path. But we can use it in error.
            )
        }
    };
    let path_vec: Vec<String> = path.trim().split(":").map(|path| path.to_string()).collect();
    Ok(path_vec)
}

fn get_bins(call: &EvaluatedCall, paths: Vec<String>) -> Result<Vec<String>, LabeledError> {
    let mut binaries = Vec::new();
    for path in paths.into_iter() {
        for entry in std::fs::read_dir(path).map_err(|e| LabeledError::new(e.to_string()))? {
            let entry = entry.map_err(|e| LabeledError::new(e.to_string()))?;
            let path = entry.path();
            if path.is_file() {
                let filename = match path.file_name() {
                    Some(val) => val,
                    None => {
                        return Err(
                            LabeledError::new("Unexpected error occured. Failed to resolve filename").with_label("Value returned None.", call.head)
                        );
                    }
                };
                let binary = match filename.to_str() {
                    Some(val) => val.to_string(),
                    None => {
                        return Err(
                            LabeledError::new("Unexpected error occured. Failed to resolve filename").with_label("Value returned None.", call.head)
                        );
                    }
                };
                binaries.push(binary);
            }
        } 
    }
    Ok(binaries)
}
pub fn dependcheck(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let deps: Vec<String> = call.rest(0)?;
    dbg!("Finding deps: {:?}", &deps);
    let path = get_path_resolved(call)?;
    dbg!("Got path: {:?}", &path);
    let mut not_found: Vec<String> = vec![];
    let bins = get_bins(call, path)?;
    dbg!("Got bins: {:?}", &bins);
    for dep in deps.iter() {
        match bins.contains(dep) {
            true => {
                dbg!("Path contains: {}", dep);
                continue;

            }
            false => {
                dbg!("Path does not contain: {}", dep);
                not_found.push(dep.to_string());
                continue;
            }
        }
    }
    if not_found.len() != 0 {
        return Err(
            LabeledError::new("Failed the dependency Check.")
                .with_label(format!("Expected Dependencies: {}, Did not find: {}", deps.join(", "), not_found.join(", ")), call.head)
        )
    } else {
        return Ok(PipelineData::Empty)
    }
}

pub struct DependencyCheck;

impl PluginCommand for DependencyCheck {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev dependcheck"
    }
    fn description(&self) -> &str {
        "Note: This is a developer command\nThis function is there to ensure dependency check. It takes in a list of names, gets path, and ensures that the command is in path."
    }
    fn signature(&self) -> nu_protocol::Signature {
        Signature::new(self.name())
        .category(Category::Custom("Developer".to_string()))
        .rest(
            "Dependencies",
            SyntaxShape::String,
            "The dependencies to check."
        )
    }
    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        dependcheck(call)
    }
}



pub struct AnyOneOf;



pub fn anyoneof(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let deps: Vec<String> = call.rest(0)?;
    dbg!("Finding deps: {:?}", &deps);
    let path = get_path_resolved(call)?;
    dbg!("Got path: {:?}", &path);
    let bins = get_bins(call, path)?;
    dbg!("Got bins: {:?}", &bins);
    for dep in deps.iter() {
        match bins.contains(dep) {
            true => {
                dbg!("Path contains: {}", dep);
                return Ok(PipelineData::value(Value::string(dep, call.head), None))
            }
            false => {
                dbg!("Path does not contain: {}", dep);
                continue;
            }
        }
    }
    return Err(
            LabeledError::new("Failed the dependency.")
                .with_label(format!("Expected Any one of: {}, Could not find any.", deps.join(", ")), call.head)
        )
}


impl PluginCommand for AnyOneOf {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev anyoneof"
    }
    fn description(&self) -> &str {
        "Note: This is a developer command\nThis function is there to get any one of the commands listed. It takes in a list of names, gets path, and returns the first program that is in the path from the list."
    }
    fn signature(&self) -> nu_protocol::Signature {
        Signature::new(self.name())
            .category(Category::Custom("Developer".to_string()))
            .rest(
                "Dependencies",
                SyntaxShape::String,
                "List of Optional Dependencies"
            )
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
            anyoneof(call)
        }
}