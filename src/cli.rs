use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "near_pump_listener")]
#[command(about = "A Near block listener", long_about = None)]
pub struct Cli {
    #[arg(long, default_value = "testnet")]
    pub network: Networks,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Networks {
    Testnet,
    Mainnet,
}
