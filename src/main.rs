// src/main.rs
/*
 * Main executable for TradingPilot
 */

use clap::Parser;
use tradingpilot::{Result, run};

#[derive(Parser)]
#[command(version, about = "TradingPilot - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
