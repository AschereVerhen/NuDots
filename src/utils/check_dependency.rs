use std::path::PathBuf;
use nu_protocol::LabeledError;
use which::which;

pub fn check_depends(depends: Vec<String>) -> Result<Vec<PathBuf>, LabeledError> {
    let mut vec: Vec<PathBuf> = Vec::new();
    let mut not_found: Vec<String> = Vec::new();
    for element in depends.into_iter() {
        if let Ok(e) = which(&element) {
            vec.push(e);
        } else {
            not_found.push(element);
        }
    }
    if !not_found.is_empty() {
        return Err(LabeledError::new(format!("Could not find dependencies: {}", not_found.join(", "))));
    }
    Ok(vec)
}