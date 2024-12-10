use clap::Parser;


#[derive(Debug, Clone, Copy)]
pub enum AsusProfile {
    Performance,
    Balanced,
    Quiet,
}

#[derive(Debug, Parser)]
pub struct ProfileOpts {
    #[arg(short, long, default_value = "false",)]
    pub get: bool,
}