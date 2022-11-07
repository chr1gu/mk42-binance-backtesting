use chrono::NaiveDate;
use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::{panic, process, time::Instant};
mod cli;
mod klines;
mod symbols;
mod types;

fn main() {
    let time = Instant::now();
    let args = cli::Cli::parse();
    if args.version {
        return println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    if !args.continue_on_error {
        panic::set_hook(Box::new(|panic_info| {
            println!("{}", panic_info.to_string().red());
            process::exit(1);
        }));
    }

    let start_date = if let Some(start_date) = &args.start_date {
        Some(NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap())
    } else {
        None
    };

    let end_date = if let Some(end_date) = args.end_date {
        Some(NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap())
    } else {
        None
    };

    let all_symbols = symbols::get_symbols().unwrap();
    if args.list_symbols {
        return println!("{}", all_symbols.join(", "));
    }

    let symbol_filter = Regex::new(&args.symbol).unwrap();
    let symbols: Vec<String> = all_symbols
        .iter()
        .filter(|s| symbol_filter.is_match(s))
        .map(|s| s.to_owned())
        .collect();

    println!("{} symbols found: {}", symbols.len(), symbols.join(", "));
    symbols.iter().for_each(|symbol| {
        klines::get_klines(
            symbol,
            &args.interval,
            &start_date,
            &end_date,
            &args.path,
            &args.force,
        );
    });

    println!("Finished after {:.2?}", time.elapsed(),);
}
