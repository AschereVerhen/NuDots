use crate::prelude::*;
pub fn uninstall(call: &EvaluatedCall, packages: Vec<String>, os: OS, no_confirm: bool, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, no_confirm, PkgOp::Uninstall)
}

fn signature() -> Signature {
    Signature::new("nupkg uninstall")
        .category(Category::Custom("Package Management".to_string()))
        .rest(
            "Packages",
            SyntaxShape::String,
            "The Packages to uninstall."
        )
        .add_help()
        .switch("yes", "Skip Confirmation", Some('y'))
        .input_output_type(Type::Any, Type::Nothing)
        .allows_unknown_args()
}

fn examples() -> Vec<Example<'static>> {
    vec![
        Example {
            example: "nupkg uninstall hyprland",
            description: "Easily uninstall packages without having to memorize your distro's flags.",
            result: None,
        },
        Example {
            example: "['waybar', 'startx'] | nupkg uninstall",
            description: "Also takes in from stdin.",
            result: None,
        }
    ]
}

fn call_uninstall(
    engine: EngineInterface,
    call: EvaluatedCall,
    input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    // Collect packages from rest arguments
    let mut packages: Vec<String> = call.rest(0)?;

    // Collect packages from pipeline input
    for value in input {
        if let Ok(pkg_str) = value.as_str() {
            packages.push(pkg_str.to_string());
        }
    }

    let no_confirm: bool = call.has_flag("yes")?;
    let os = detect_os_raw();

    uninstall(&call, packages, os, no_confirm, &engine)?;

    Ok(PipelineData::Empty)
}

#[plugin_command(
    name = "nupkg uninstall",
    plugin = Nudo,
    description = "Allows you to uninstall packages os-agnostically",
    signature = signature(),
    examples = examples(),
    run = call_uninstall
)]
pub struct Uninstall;