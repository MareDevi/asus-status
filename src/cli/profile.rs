use std::{fmt::{self, Display, Formatter}, str::FromStr};
use clap::Parser;


#[derive(Debug, Clone, Copy)]
pub enum AsusProfile {
    Performance,
    Balanced,
    Quiet,
}

#[derive(Debug, Parser)]
pub struct ProfileOpts {
    #[command(subcommand)]
    pub profile_subcommand: ProfileSubCommand,
}

#[derive(Debug, Parser)]
pub enum ProfileSubCommand {
    #[clap(name = "get", about = "Get the active profile")]
    Get,
    #[clap(name = "set", about = "Set to the next profile")]
    Set,
}

impl FromStr for AsusProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "performance" => Ok(AsusProfile::Performance),
            "balanced" => Ok(AsusProfile::Balanced),
            "quiet" => Ok(AsusProfile::Quiet),
            _ => Err(format!("Invalid profile: {}", s)),
        }
    }
}

impl Display for AsusProfile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            AsusProfile::Performance => "Performance",
            AsusProfile::Balanced => "Balanced",
            AsusProfile::Quiet => "Quiet",
        })
    }
}