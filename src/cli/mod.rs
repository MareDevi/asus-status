use clap::{command, Parser};

mod profile;
mod graphics;

pub use profile::*;
pub use graphics::*;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[clap(name = "profile", about = "Get or set the active profile")]
    Profile(ProfileOpts),
    #[clap(name = "graphics", about = "Get or set the active GPU mode")]
    Graphics(GraphicsOpts),
    #[clap(name = "info", about = "Get the current computer information")]
    Info,
}