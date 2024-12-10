use std::process::Command;
use crate::utils::extract_command_output;

pub fn get_graphics_mode() -> String {
    let output = extract_command_output(Command::new("supergfxctl").arg("-g"));
    return output;
}