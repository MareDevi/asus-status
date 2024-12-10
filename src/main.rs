use clap::Parser;
use serde::Serialize;
use asus_status::{get_graphics_mode, get_profile_info, set_to_next_graphics_mode, set_to_next_profile, GraphicsSubCommand, Opts, ProfileSubCommand, SubCommand};

#[derive(Debug, Serialize)]
struct Profile {
    text: String,
}

#[derive(Debug, Serialize)]
struct Graphics {
    text: String,
}

#[derive(Debug, Serialize)]
struct Info {
    text: String,
}

fn main() {
    let opts = Opts::parse();

    let mut profile: Option<Profile> = None;
    let mut graphics: Option<Graphics> = None;
    let mut info: Option<Info> = None;

    match opts.cmd {
        SubCommand::Profile(profile_opts) => {
            match profile_opts.profile_subcommand {
                ProfileSubCommand::Get => {
                    profile = Some(Profile {
                        text: "󰚗 :".to_owned() + &get_profile_info()[2],
                    });
                },
                ProfileSubCommand::Set => {
                    set_to_next_profile();
                },
            }
        },
        SubCommand::Graphics(graphics_opts) => {
            match graphics_opts.graphics_subcommand {
                GraphicsSubCommand::Get => {
                    graphics = Some(Graphics {
                        text: "󰿠 :".to_owned() + &get_graphics_mode(),
                    });
                },
                GraphicsSubCommand::Set => {
                    set_to_next_graphics_mode();
                },
            }
        },
        SubCommand::Info => {
            info = Some(Info {
                text: "󰹑 ".to_owned() + &get_profile_info()[0],
            });
        },
    }

    //判断输出，并将其转化为json并打印
    if let Some(profile) = profile {
        println!("{}", serde_json::to_string(&profile).unwrap());
    } else if let Some(graphics) = graphics {
        println!("{}", serde_json::to_string(&graphics).unwrap());
    } else if let Some(info) = info {
        println!("{}", serde_json::to_string(&info).unwrap());
    }

}
