use crate::{
    klines::{self, Kline},
    trading_signal::TradingSignal,
};
use anyhow::{Ok, Result};
use chrono::{Duration, NaiveDate};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use colored::Colorize;
use csv::Reader;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use log::{debug, info};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Write,
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn test(
    symbol_filter: Regex,
    start_date: NaiveDate,
    end_date: NaiveDate,
    data_dir: PathBuf,
    verbosity: Verbosity<InfoLevel>,
) -> Result<f64> {
    let progress = MultiProgress::new();
    let logger = env_logger::Builder::new()
        .filter_level(verbosity.log_level_filter())
        .build();
    LogWrapper::new(progress.clone(), logger)
        .try_init()
        .unwrap();

    let duration = end_date.signed_duration_since(start_date).num_days() as u64;
    let progress_bar = ProgressBar::new(duration);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    let mut signals_by_symbol: HashMap<String, TradingSignal> = HashMap::new();
    let symbol_path_regex = Regex::new(r"(?P<symbol>\w+)-1m").unwrap();

    let mut day = start_date;
    while day <= end_date {
        debug!("Processing {:?}", day);

        let dir = data_dir.join(day.format("%Y/%m/%d").to_string());
        let files: Vec<DirEntry> = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect();

        // add trading signals for newly discovered symbols
        for file in &files {
            let filepath: PathBuf = file.path();
            let filepath_str = filepath.to_str().unwrap();
            let Some(matches) = symbol_path_regex.captures(filepath_str) else {
                panic!("{}", format!("Can't parse symbol from path {filepath_str}"));
            };

            let symbol = matches.name("symbol").unwrap().as_str().to_string();
            if !symbol_filter.is_match(&symbol) {
                continue;
            }

            if !signals_by_symbol.contains_key(&symbol) {
                debug!("Wild symbol {} appeared", symbol);
                signals_by_symbol.insert(symbol.clone(), TradingSignal::new(symbol));
            }
        }

        // process trading signals
        signals_by_symbol
            .par_iter_mut()
            .for_each(|(symbol, signal)| {
                let filepath = dir.join(format!("{symbol}-1m-{}.csv", day.format("%Y-%m-%d")));
                if filepath.exists() {
                    let mut reader = Reader::from_path(&filepath).unwrap();
                    reader.set_headers(klines::csv_headers());

                    for result in reader.deserialize() {
                        let kline: Kline = result.unwrap();
                        signal.update(kline).unwrap();
                    }
                }
            });

        day += Duration::days(1);
        progress_bar.inc(1);
    }

    let mut total_performance = 0.0;
    let mut total_updates = 0;
    let mut total_symbols = 0;
    let mut total_trades = 0;

    // finalize each signal
    signals_by_symbol.iter_mut().for_each(|(_, signal)| {
        signal.finalize().unwrap();
    });

    // sort by performance
    let mut signals: Vec<TradingSignal> = signals_by_symbol.into_values().collect();
    signals.sort_by(|a, b| {
        b.stats
            .performance
            .partial_cmp(&a.stats.performance)
            .unwrap()
    });

    // print stats
    signals.iter().for_each(|signal| {
        total_symbols += 1;
        total_updates += signal.stats.updates;
        total_performance += signal.stats.performance;
        total_trades += signal.stats.total_sells;
        info!("{}", signal);
    });

    let updates = total_updates
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("'");

    let performance = if total_performance > 0.0 {
        format!("{}%", total_performance.round()).green()
    } else {
        format!("{}%", total_performance.round()).red()
    };

    info!("{total_symbols} Symbols discovered and {updates} klines processed.",);
    info!("Total performance: {performance}, trades: {total_trades}");
    Ok(total_performance)
}
