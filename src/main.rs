use anyhow::Result;
use clap::Parser;
use cli::Commands;
use date::DateString;
mod fetch_command;
mod progress;
mod symbols;
mod types;
mod klines;
mod date;
mod cli;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    match args.command {
        Commands::Fetch { interval, path, shared_args  } => {
            let start_date = shared_args.start_date.as_date();
            let end_date = shared_args.end_date.as_date();
            fetch_command::fetch(interval, shared_args.symbol, start_date, end_date, path, args.verbose)?
        },
        Commands::Run { shared_args } => {
            let start_date = shared_args.start_date.as_date();
            let end_date = shared_args.end_date.as_date();
            println!("Hi {:?} {:?}", start_date, end_date)
        },
    }

    Ok(())
}

