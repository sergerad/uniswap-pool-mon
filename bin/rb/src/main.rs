use clap::{Parser, Subcommand};

mod commands;
use commands::{erc20_symbol, pool_tick_spacing, Erc20Args, Erc20Commands, PoolArgs, PoolCommands};

#[derive(Debug, Parser)]
#[command(name = "rb")]
#[command(about = "R-bitrage Bot", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Erc20(Erc20Args),
    Pool(PoolArgs),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Erc20(args) => match args.command {
            Some(Erc20Commands::Symbol) => erc20_symbol().await,
            None => unreachable!(),
        },
        Commands::Pool(args) => match args.command {
            Some(PoolCommands::TickSpacing) => pool_tick_spacing().await,
            None => unreachable!(),
        },
    }
}