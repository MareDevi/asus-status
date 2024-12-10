use clap::Parser;

#[derive(Debug, Parser)]
pub struct GraphicsOpts {
    #[arg(short, long, default_value = "false")]
    pub get: bool,
}