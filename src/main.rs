use std::time::Instant;

use anyhow::{Ok, Result};
use clap::Parser;
use cli::Commands;
use date::DateString;
use log::info;
use regex::Regex;
mod cli;
mod date;
mod fetch_command;
mod klines;
mod progress;
mod symbols;
mod test_command;
mod trading_signal;
mod types;

fn main() -> Result<()> {
    let start = Instant::now();
    let args = cli::Cli::parse();
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    match args.command {
        Commands::Fetch {
            interval,
            path,
            start_date,
            end_date,
            symbol,
        } => {
            let start_date = start_date.try_parse_date();
            let end_date = end_date.try_parse_date();
            fetch_command::fetch(interval, symbol, start_date, end_date, path, args.verbose)?
        }
        Commands::Test {
            symbol,
            path,
            start_date,
            end_date,
        } => {
            let symbol_regex = Regex::new(&symbol).unwrap();
            let start_date = start_date.parse_date();
            let end_date = end_date.parse_date();
            test_command::test(symbol_regex, start_date, end_date, path, args.verbose)?;
            return Ok(());
        }
    }

    let duration = start.elapsed();
    info!("Finished after {:?}", duration);
    Ok(())
}
