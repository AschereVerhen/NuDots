use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Value
};
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};
use std::io::Read;

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



pub struct AnyOneOf;



pub fn anyoneof(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let deps: Vec<String> = call.rest(0)?;
    // dbg!("Finding deps: {:?}", &deps);
    let path = get_path_resolved(call)?;
    // dbg!("Got path: {:?}", &path);
    let bins = get_bins(call, path)?;
    // dbg!("Got bins: {:?}", &bins);
    for dep in deps.iter() {
        match bins.contains(dep) {
            true => {
                // dbg!("Path contains: {}", dep);
                return Ok(PipelineData::value(Value::string(dep, call.head), None))
            }
            false => {
                // dbg!("Path does not contain: {}", dep);
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
            .add_help()
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

//Now implimenting detect_os.
pub struct DetectOs;

pub fn detect_os(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let os = std::env::consts::OS;
    if os != "linux" {
        return Ok(PipelineData::value(Value::string(os, call.head), None));
    }
    //Now, we first of all open /etc/os-release now that we *know* the system is linux.
    let mut file = std::fs::File::open("/etc/os-release").map_err(|e| LabeledError::new(e.to_string()))?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents).map_err(|e| LabeledError::new(e.to_string()))?;
    let distro: Option<String> = contents.lines().find(|line| line.starts_with("ID=")).and_then(|line| line.split("=").nth(1)).map(|s| s.trim_matches('"').to_string());
    return Ok(PipelineData::value(Value::string(distro.unwrap_or("Unknown Linux".to_string()), call.head), None));
}

impl PluginCommand for DetectOs {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev detectos"
    }
    fn description(&self) -> &str {
        "This command detects and returns Your Operating System. And if its Linux Or BSD, it will also return the Distro."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .add_help()
            .category(Category::Custom("Developer".to_string()))
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
            detect_os(call)
    }
}

pub fn args_required(call: &EvaluatedCall, min_args: u16) -> Result<(), LabeledError> {
    let arglist: Vec<String> = call.rest(0)?;
    if arglist.len() < min_args.into() {
        return Err(
            LabeledError::new("Minimum Args not matched.")
                .with_label(format!("Required Args: {}, Found args: {}", min_args, arglist.len()), call.head)
        )
    }
    return Ok(())

}

pub struct ArgsRequired;

impl PluginCommand for ArgsRequired {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev args_required"
    }
    fn description(&self) -> &str {
        "This command detects if the user supplied enough args or not. If not, it errors out early."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Developer".to_string()))
            .required("Arg", SyntaxShape::Int, "The Minimum Number of Args")
            .add_help()
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
        let min_args = call.req(0)?;
        args_required(call, min_args)?;
        Ok(PipelineData::Empty)
    }
}


pub struct Run;

pub fn run(call: &EvaluatedCall, cmd: String, arguments: Vec<String>) -> Result<(), LabeledError> {
    use std::process::{Command, Stdio};
    let args;
    let mut command_raw: String = cmd.to_string();
    if arguments.len() != 0 {
        args = arguments
    } else {
        let c: Vec<&str> = cmd.split_whitespace().collect();
        command_raw = c[0].to_string();
        args = c[1..].into_iter().map(|element| element.to_string()).collect::<Vec<String>>();
    }
    let mut command = Command::new(command_raw);
    command.args(args);
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.stderr(Stdio::inherit());
    match command.output() {
        Err(e) => {
            return Err(
            LabeledError::new(format!("An Error occured: {:?}", e))
                .with_label("This Command", call.head)
            )
        },
        Ok(_) => return Ok(())
    }
}

impl PluginCommand for Run {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo dev run"
    }
    fn description(&self) -> &str {
        "Runs a command with an args list. And then Provides a labelled Error if it errored out."
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Developer".to_string()))
            .required("Command", SyntaxShape::String, "The Command to execute")
            .rest(
                "Arguments",
                SyntaxShape::String,
                "The list of arguments to run the command with."
            )
            .add_help()
            .allows_unknown_args()
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
            call: &EvaluatedCall,
            _input: PipelineData,
        ) -> Result<PipelineData, LabeledError> {
        let command = call.req(0)?;
        let arguments = call.rest(1)?;
        run(call, command, arguments)?;
        Ok(PipelineData::Empty)
    }
}