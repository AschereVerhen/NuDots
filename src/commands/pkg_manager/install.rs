use crate::prelude::*;
pub fn install(call: &EvaluatedCall, packages: Vec<String>, os: OS, no_confirm: bool, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, no_confirm, PkgOp::Install)
}
fn examples<'a>() -> Vec<nu_protocol::Example<'a>> {
    vec![
        nu_protocol::Example {
            example: "nupkg install hyprland qs emerge",
            description: "Easily install packages without having to memorize your distro's flags.",
            result: None,
        },
        nu_protocol::Example {
            example: "['waybar', 'startx', 'bluetoothctl'] | nupkg install",
            description: "Also takes in from stdin.",
            result: None,
        }
    ]
}
fn signature() -> Signature {
    Signature::new(Install.name())
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
fn call_install(
        engine: nu_plugin::EngineInterface,
        call: EvaluatedCall,
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
    install(&call, packages, os, no_confirm, &engine)?;
    Ok(PipelineData::Empty)
}

#[plugin_command(
plugin = Nudo,
name = "nupkg install",
run = call_install,
signature = signature(),
description = "Allows you to install packages os-agnostically",
examples = examples(),
)]
pub struct Install;