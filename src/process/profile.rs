use regex::Regex;
use std::process::Command;
use crate::utils::extract_command_output;

pub fn extract_info(pattern: &str, text: &str) -> String {
    Regex::new(pattern)
        .unwrap()
        .captures(text)
        .and_then(|caps| caps.get(1))
        .map_or("Unknown".to_string(), |m| m.as_str().to_string())
}

pub fn get_profile_info() -> Vec<String> {
    let info_output = extract_command_output(Command::new("asusctl").arg("-v"));
    let profile_output = extract_command_output(Command::new("asusctl").arg("profile").arg("-p"));

    return vec![
        extract_info(r"Product family:\s*(.*)", &info_output),
        extract_info(r"Board name:\s*(.*)", &info_output),
        extract_info(r"Active profile is\s*(.*)", &profile_output),
    ];
}