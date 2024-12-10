use std::str::FromStr;
use std::fmt::Display;
use clap::Parser;

//[Integrated, Hybrid, AsusMuxDgpu]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphicsMode {
    Integrated,
    Hybrid,
    AsusMuxDgpu,
}

#[derive(Debug, Parser)]
pub struct GraphicsOpts {
    #[command(subcommand)]
    pub graphics_subcommand: GraphicsSubCommand,
}

#[derive(Debug, Parser)]
pub enum GraphicsSubCommand {
    #[clap(name = "get", about = "Get the active graphics mode")]
    Get,
    #[clap(name = "set", about = "Set the graphics mode")]
    Set
}

impl FromStr for GraphicsMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "integrated" => Ok(GraphicsMode::Integrated),
            "hybrid" => Ok(GraphicsMode::Hybrid),
            "asusmuxdgpu" => Ok(GraphicsMode::AsusMuxDgpu),
            _ => Err(format!("Invalid graphics mode: {}", s)),
        }
    }
}

impl Display for GraphicsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            GraphicsMode::Integrated => "Integrated",
            GraphicsMode::Hybrid => "Hybrid",
            GraphicsMode::AsusMuxDgpu => "AsusMuxDgpu",
        })
    }
}