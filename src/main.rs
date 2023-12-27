use std::time::Instant;

use anyhow::{Ok, Result};
use chrono::{Duration, NaiveDate};
use clap::Parser;
use cli::Commands;
use colored::Colorize;
use crossbeam::channel;
use date::DateString;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
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
mod visualize_command;

fn main() -> Result<()> {
    let start = Instant::now();
    let args = cli::Cli::parse();
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    let progress = MultiProgress::new();
    let logger = env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .build();
    LogWrapper::new(progress.clone(), logger)
        .try_init()
        .unwrap();

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
            test_command::test(&symbol_regex, &start_date, &end_date, path, &progress)?;
        }
        Commands::Visualize {
            symbol,
            path,
            start_date,
            end_date,
        } => {
            let symbol_regex = Regex::new(&symbol).unwrap();
            let start_date = start_date.parse_date();
            let end_date = end_date.parse_date();
            visualize_command::visualize(&symbol_regex, &start_date, &end_date, path, &progress)?;
        }
        Commands::TestVariants {
            symbol,
            path,
            start_date,
            end_date,
        } => {
            let start_date = start_date.parse_date();
            let end_date = end_date.parse_date();
            let total_days = end_date.signed_duration_since(start_date).num_days();

            // TODO: move end_date around too, to test various ranges and increase length to see results early

            let start_dates: Vec<NaiveDate> = (0..total_days)
                .map(|i| start_date + Duration::days(i))
                .collect();
            let path = path.as_path();
            let (sender, receiver) = channel::unbounded();
            start_dates.par_iter().for_each(|start_date| {
                let symbol_regex = Regex::new(&symbol).unwrap();
                let performance = test_command::test(
                    &symbol_regex,
                    start_date,
                    &end_date,
                    path.to_path_buf(),
                    &progress,
                )
                .unwrap();
                sender.send(performance).unwrap();
            });
            drop(sender);

            let variations = receiver.len() as f64;
            let performances = receiver.into_iter().sum::<f64>();
            let average_performance = performances / variations;
            let performance_label = if average_performance > 0.0 {
                format!("{}%", average_performance.round()).green()
            } else {
                format!("{}%", average_performance.round()).red()
            };
            info!("Average performance from {start_date} to {end_date} with {variations} variations: {performance_label}",)
        }
    }

    let duration = start.elapsed();
    info!("Finished after {:?}", duration);
    Ok(())
}
