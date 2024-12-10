use std::process::Command;
use notify_rust::Notification;

use crate::utils::extract_command_output;
use crate::GraphicsMode;

pub fn get_graphics_mode() -> String {
    let output = extract_command_output(Command::new("supergfxctl").arg("-g"));
    return output;
}

pub fn set_to_next_graphics_mode() {
    let current_graphics_mode = get_graphics_mode();
    let next_graphics_mode = match current_graphics_mode.as_str() {
        "Integrated" => GraphicsMode::Hybrid,
        "Hybrid" => GraphicsMode::AsusMuxDgpu,
        "AsusMuxDgpu" => GraphicsMode::Integrated,
        _ => GraphicsMode::Integrated,
    };

    Command::new("supergfxctl")
        .arg("-m")
        .arg(format!("{}", next_graphics_mode))
        .output()
        .expect("Failed to set graphics mode");

    //if the next graphics mode is AsusMuxDgpu, we need to show a notification to remind user to reboot
    //else we can just remind them to logout to take effect
    if next_graphics_mode == GraphicsMode::AsusMuxDgpu {
        Notification::new()
            .summary("AsusMuxDgpu mode set")
            .body("Please reboot to take effect")
            .show()
            .expect("Failed to show notification");
    } else {
        Notification::new()
            .summary("Graphics mode set")
            .body(format!("GPU mode setted from {} to {}\nPlease logout to take effect",current_graphics_mode, next_graphics_mode).as_str())
            .show()
            .expect("Failed to show notification");
    }
}