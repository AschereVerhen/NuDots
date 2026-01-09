use crate::prelude::*;
pub fn update(call: &EvaluatedCall, packages: Vec<String>, os: OS, no_confirm: bool, engine: &EngineInterface) -> Result<(), LabeledError> {
    create_command(call, engine, packages, os, no_confirm, PkgOp::Update)
}

fn signature() -> Signature {
    Signature::new(Update.name())
        .category(Category::Custom("Package Management".to_string()))
        .rest(
            "Packages",
            SyntaxShape::String,
            "The Packages to update. This is entirely Optional."
        )
        .add_help()
        .switch("yes", "Skip Confirmation", Some('y'))
        .input_output_type(Type::Any, Type::Nothing)
        .allows_unknown_args()
}

fn examples() -> Vec<Example<'static>> {
    vec![
        Example {
            example: "nupkg update hyprland",
            description: "Easily update packages without having to memorize your distro's flags.",
            result: None,
        },
        Example {
            example: "['waybar', 'startx'] | nupkg update",
            description: "Also takes in from stdin.",
            result: None,
        }
    ]
}

fn call_update(
    engine: EngineInterface,
    call: EvaluatedCall,
    input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let mut packages: Vec<String> = call.rest(0)?;

    // Collect from pipeline
    for value in input {
        if let Ok(pkg_str) = value.as_str() {
            packages.push(pkg_str.to_string());
        }
    }

    let no_confirm: bool = call.has_flag("yes")?;
    let os = detect_os_raw();

    update(&call, packages, os, no_confirm, &engine)?;

    Ok(PipelineData::Empty)
}

#[plugin_command(
    name = "nupkg update",
    plugin = Nudo,
    description = "Allows you to update packages os-agnostically",
    signature = signature(),
    examples = examples(),
    run = call_update
)]
pub struct Update;