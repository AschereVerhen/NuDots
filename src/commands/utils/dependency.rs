use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};

use crate::{Nudo, errors::MyError};
//Firstly, implimenting the dependency_check program.
//We do not want to add this to any nu api. Except error making.

//firstly, get path variable

pub fn get_path_resolved_raw() -> Result<Vec<String>, MyError>{
    let path = if let Some(value) = std::env::var_os("PATH") {
        value
    } else {
        //We throw an error.
        return Err(
            MyError::PathNotInitialized
        );
    };
    let path = match path.into_string() {
        Ok(val) => val,
        Err(_) => {
            return Err(
                MyError::PathNotValid
            )
        }
    };
    let path_vec: Vec<String> = path.trim().split(":").map(|path| path.to_string()).collect();
    Ok(path_vec)
}
//This should not be public. NameSpace Clutter.
pub fn get_path_resolved(call: &EvaluatedCall) -> Result<Vec<String>, LabeledError>{
    let path = get_path_resolved_raw();
    match path {
        Ok(val) => Ok(val),
        Err(MyError::PathNotInitialized) => {
            return Err(
            LabeledError::new("Could not find path")
                .with_label("$env.PATH does not seem to be set", call.head)
        )},
        Err(MyError::PathNotValid) => {
            return Err(
                LabeledError::new("Invalid PATH")
            )
        },
        _ => Err(LabeledError::new("Unknown/Non-Covered Error.")), //This shouldnt come. Since we do not return Any other one.
    }
}

pub fn get_bins_raw(paths: Vec<String>) -> Result<Vec<String>, MyError> {
    let mut binaries = Vec::new();
    for path in paths.into_iter() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let filename = match path.file_name() {
                    Option::Some(val) => val,
                    Option::None => {
                        return Err(
                            MyError::UnexpectedError {text: "Failed to Parse filename".to_string()}
                        );
                    }
                };
                let binary = match filename.to_str() {
                    Option::Some(val) => val.to_string(),
                    Option::None => {
                        return Err(
                            MyError::UnexpectedError {text: "Failed to Convert filename to UTF-8.".to_string()}
                        );
                    }
                };
                binaries.push(binary);
            }
        } 
    }
    Ok(binaries)
}


pub fn get_bins(call: &EvaluatedCall, paths: Vec<String>) -> Result<Vec<String>, LabeledError> {
    let binaries = get_bins_raw(paths);
    match binaries {    
        Ok(binaries) => return Ok(binaries),
        Err(MyError::UnexpectedError { text }) => {
            return Err(
                LabeledError::new(text).with_label("Value returned None.", call.head)
            )
        },
        _ => {
            return Err(
                LabeledError::new("Unknown Error Occured").with_label("Occured here", call.head).with_help("Please create a Github Issue.")
            )
        }
    };
}
pub fn dependcheck(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let deps: Vec<String> = call.rest(0)?;
    // dbg!("Finding deps: {:?}", &deps);
    let path = get_path_resolved(call)?;
    // dbg!("Got path: {:?}", &path);
    let mut not_found: Vec<String> = vec![];
    let bins = get_bins(call, path)?;
    // dbg!("Got bins: {:?}", &bins);
    for dep in deps.iter() {
        match bins.contains(dep) {
            true => {
                // dbg!("Path contains: {}", dep);
                continue;

            }
            false => {
                // dbg!("Path does not contain: {}", dep);
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
        return Ok(PipelineData::Empty) //This returns nothing.
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
        .input_output_type(Type::list(Type::String), Type::Nothing)
        .add_help()
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


