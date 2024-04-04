mod balance;
mod busses;
mod claim;
mod cu_limits;
#[cfg(feature = "admin")]
mod initialize;
mod mine;
mod register;
mod rewards;
mod send_and_confirm;
mod treasury;
#[cfg(feature = "admin")]
mod update_admin;
#[cfg(feature = "admin")]
mod update_difficulty;
mod utils;

use std::sync::Arc;

use clap::{command, Parser, Subcommand};
use solana_sdk::signature::{read_keypair_file, Keypair};

struct Miner {
    pub keypair_filepath: Option<String>,
    pub priority_fee: u64,
    pub cluster: String,
}

#[derive(Parser, Debug)]
#[command(about, version)]
struct Args {
    #[arg(
        long,
        value_name = "NETWORK_URL",
        help = "Network address of your RPC provider",
        default_value = "https://api.mainnet-beta.solana.com"
    )]
    rpc: String,

    #[arg(
        long,
        value_name = "KEYPAIR_FILEPATH",
        help = "Filepath to keypair to use"
    )]
    keypair: Option<String>,

    #[arg(
        long,
        value_name = "MICROLAMPORTS",
        help = "Number of microlamports to pay as priority fee per transaction",
        default_value = "0"
    )]
    priority_fee: u64,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Fetch the Ore balance of an account")]
    Balance(BalanceArgs),

    #[command(about = "Fetch the distributable rewards of the busses")]
    Busses(BussesArgs),

    #[command(about = "Mine Ore using local compute")]
    Mine(MineArgs),

    #[command(about = "Claim available mining rewards")]
    Claim(ClaimArgs),

    #[command(about = "Fetch your balance of unclaimed mining rewards")]
    Rewards(RewardsArgs),

    #[command(about = "Fetch the treasury account and balance")]
    Treasury(TreasuryArgs),

    #[cfg(feature = "admin")]
    #[command(about = "Initialize the program")]
    Initialize(InitializeArgs),

    #[cfg(feature = "admin")]
    #[command(about = "Update the program admin authority")]
    UpdateAdmin(UpdateAdminArgs),

    #[cfg(feature = "admin")]
    #[command(about = "Update the mining difficulty")]
    UpdateDifficulty(UpdateDifficultyArgs),
}

#[derive(Parser, Debug)]
struct BalanceArgs {
    #[arg(
        // long,
        value_name = "ADDRESS",
        help = "The address of the account to fetch the balance of"
    )]
    pub address: Option<String>,
}

#[derive(Parser, Debug)]
struct BussesArgs {}

#[derive(Parser, Debug)]
struct RewardsArgs {
    #[arg(
        // long,
        value_name = "ADDRESS",
        help = "The address of the account to fetch the rewards balance of"
    )]
    pub address: Option<String>,
}

#[derive(Parser, Debug)]
struct MineArgs {
    #[arg(
        long,
        short,
        value_name = "THREAD_COUNT",
        help = "The number of threads to dedicate to mining",
        default_value = "1"
    )]
    threads: u64,
}

#[derive(Parser, Debug)]
struct TreasuryArgs {}

#[derive(Parser, Debug)]
struct ClaimArgs {
    #[arg(
        // long,
        value_name = "AMOUNT",
        help = "The amount of rewards to claim. Defaults to max."
    )]
    amount: Option<f64>,

    #[arg(
        // long,
        value_name = "TOKEN_ACCOUNT_ADDRESS",
        help = "Token account to receive mining rewards."
    )]
    beneficiary: Option<String>,
}

#[cfg(feature = "admin")]
#[derive(Parser, Debug)]
struct InitializeArgs {}

#[cfg(feature = "admin")]
#[derive(Parser, Debug)]
struct UpdateAdminArgs {
    new_admin: String,
}

#[cfg(feature = "admin")]
#[derive(Parser, Debug)]
struct UpdateDifficultyArgs {}

#[tokio::main]
async fn main() {
    // Initialize miner.
    let cluster = "https://summer-quaint-darkness.solana-mainnet.quiknode.pro/261daaec26e045c898d8eb54c7a9837a59214b17/";
    let miner = Arc::new(Miner::new(cluster.to_string(), 5000, Some("/Users/xbtg-/ore/ls/ore/l.json".to_string())));
    miner.mine(8).await;
}

impl Miner {
    pub fn new(cluster: String, priority_fee: u64, keypair_filepath: Option<String>) -> Self {
        Self {
            keypair_filepath,
            priority_fee,
            cluster,
        }
    }

    pub fn signer(&self) -> Keypair {
        match self.keypair_filepath.clone() {
            Some(filepath) => read_keypair_file(filepath).unwrap(),
            None => panic!("No keypair provided"),
        }
    }
}
