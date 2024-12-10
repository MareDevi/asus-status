use regex::Regex;
use std::process::Command;
use notify_rust::Notification;
use crate::{utils::extract_command_output, AsusProfile};

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

pub fn set_to_next_profile() {
    let current_profile = get_profile_info().get(2).cloned().unwrap_or_else(|| "Unknown".to_string());
    let profile = match current_profile.as_str() {
        "Performance" => AsusProfile::Balanced,
        "Balanced" => AsusProfile::Quiet,
        "Quiet" => AsusProfile::Performance,
        _ => AsusProfile::Performance,
    };
    let output = Command::new("asusctl")
        .arg("profile")
        .arg("-P")
        .arg(profile.to_string())
        .output();

    match output {
        Ok(output) if output.status.success() => {
            Notification::new()
                .summary("Profile")
                .body(&format!("Successfully set profile from {} to {}", current_profile, profile))
                .show()
                .unwrap();
        }
        Ok(output) => {
            Notification::new()
                .summary("Profile")
                .body(&format!("Failed to set to next profile: {}", String::from_utf8_lossy(&output.stderr)))
                .show()
                .unwrap();
        }
        Err(e) => {
            Notification::new()
                .summary("Profile")
                .body(&format!("Failed to set to next profile: {}", e))
                .show()
                .unwrap();
        }
    }
}