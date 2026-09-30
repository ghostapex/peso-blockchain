use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "peso-chain", version, about = "PESO blockchain prototype")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Balance { account: String },
    Transfer { from: String, to: String, amount: u64 },
    Status,
}
