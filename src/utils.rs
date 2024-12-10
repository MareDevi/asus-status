use std::process::Command;

pub fn extract_command_output(command: &mut Command) -> String {
    String::from_utf8_lossy(
        &command.output().expect("Failed to execute command").stdout,
    )
    .trim()
    .to_string()
}