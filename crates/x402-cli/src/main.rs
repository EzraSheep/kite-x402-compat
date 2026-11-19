//! `kite-x402-compat` command line entry point.

use clap::Parser;

/// Black-box compatibility test kit for x402 V2 services, with a Kite profile.
#[derive(Debug, Parser)]
#[command(name = "kite-x402-compat", version, about)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
}
