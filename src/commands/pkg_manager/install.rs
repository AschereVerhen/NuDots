//This is for pkg management system.
use nu_plugin::{
    EvaluatedCall,
    PluginCommand,
};
use nu_protocol::{
    Category, LabeledError, PipelineData, Signature, SyntaxShape, Type
};
use crate::Nudo;
use crate::commands::utils::detectos::detect_os_raw;
use crate::commands::utils::anyoneof::anyoneof_raw;
pub struct Install;

fn get_os() -> Result<String, LabeledError> {
    let os = detect_os_raw().map_err(|e| LabeledError::new(e.to_string()))?;
    Ok(os)
} //Resolves to Text-Based OS.

pub fn install(call: &EvaluatedCall, packages: Vec<String>, os: String, no_confirm: bool) -> Result<(), LabeledError> {
    let pkg_args: Vec<&str>;//Note, we need not handle the arguments from user through allows_unknown_args. That will be
    //Automatically handled in packages's expansion.
    let manager: String;

    match os.as_str() {
        "arch" => {
            let pkg_manage_list = vec!["paru".to_string(), "yay".to_string(), "pacman".to_string()];
            if let Ok(value) = anyoneof_raw(&pkg_manage_list) {
                manager = value;
            } else {
                return Err(
                    LabeledError::new("Your system is detected as an arch-based system; but pacman is not found.")
                        .with_label("Dependency Check Failed", call.head)
                        .with_help("Maybe add pacman to the PATH if it wasnt or install it?")
                )
            }
            if no_confirm {
                pkg_args = vec!["-S", "--noconfirm"]
            } else {
                pkg_args = vec!["-S"]
            }
        },
        "gentoo" => {
            manager = "emerge".to_string();
            if no_confirm{
                pkg_args = vec!["--quiet", "--verbose"]
            } else {
                pkg_args = vec!["--verbose", "--ask"]
            }
        },
        _ => {
            return Err(LabeledError::new("Your package Manager is not Implimented yet.")
                .with_help("Please Wait or Contribute")
                .with_label("This command will not work.", call.head))
        }
    }
    use std::process::{Command, Stdio};
    let mut command;
    if manager == "pacman" || manager == "emerge" {
        let priv_manager_list = vec!["sudo".to_string(), "doas".to_string(), "run0".to_string()];
        if let Ok(val) = anyoneof_raw(&priv_manager_list) {
            command = Command::new(val);
        } else {
            return Err(
                LabeledError::new("Could not find any priviledge escalatory Tools in your system.")
                    .with_label("Dependency Check failed", call.head)
                    .with_help("Maybe install either one of sudo, doas, run0?")
            );
        }
        command.arg(manager);
    } else {
        command = Command::new(manager);
    }
    println!("pkg_args: {:?}", pkg_args);
    command.args(pkg_args); //Add arguments
    command.args(packages); //Add packages
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.stderr(Stdio::inherit());
    // if let Err(e) = command.output() {
    //     return Err(
    //         LabeledError::new(e.to_string())
    //             .with_label("Something wrong happened", call.head)
    //             .with_help("Common issues might be: NetworkIssue, Or Pkg not found in repo.")
    //     )
    // }
    
    let status = command.status().map_err(|e| LabeledError::new(e.to_string()).with_label("Failed to run Package Manager", call.head))?;

    if !status.success() {
        return Err(
            LabeledError::new("Command failed with an non-zero Exit code.")
                .with_label("Something wrong happened", call.head)
                .with_help("Common issues might be: NetworkIssue, Or Pkg not found in repo.")
        )
    }

    Ok(())
} 

impl PluginCommand for Install {
    type Plugin = Nudo;
    fn name(&self) -> &str {
        "nudo install" //Installation.
    }
    fn description(&self) -> &str {
        "Allows you to install packages os-agnostically"
    }
    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .category(Category::Custom("Package Management".to_string()))
            .rest(
                "Packages",
                SyntaxShape::String,
                "The Packages to install."
            )
            .add_help()
            .switch("yes", "Skip Confirmation", Some('y'))
            .input_output_type(Type::Any, Type::Nothing) //Takes in anything; returns nothing.
            .allows_unknown_args() //Allow people to pass pkg_manager-specific flags, like --one-shot in emerge or --overwrite="*" in pacman.
    }
    fn run(
            &self,
            _plugin: &Self::Plugin,
            _engine: &nu_plugin::EngineInterface,
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
        println!("--yes: {}", no_confirm);
        let os = get_os()?;
        install(call, packages, os, no_confirm)?;
        Ok(PipelineData::Empty)
    }
}