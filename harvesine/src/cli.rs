use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Setup(SetupArgs),
    Calculate(CalculateArgs),
}

#[derive(Args)]
pub struct SetupArgs {
    /// Generate random numbers uniformly
    #[arg(long)]
    pub uniform: bool,

    /// Number of pairs generated
    #[arg(short, default_value_t = 1000)]
    pub n: usize,
}

#[derive(Args)]
pub struct CalculateArgs {
    /// Path to input json file
    #[arg(long)]
    pub path: String,
}
