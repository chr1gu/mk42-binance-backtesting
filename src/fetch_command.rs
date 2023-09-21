use std::path::{PathBuf, Path};
use chrono::NaiveDate;
use clap_verbosity_flag::{Verbosity, InfoLevel};
use crossbeam::{channel::{self}, scope};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use crossbeam::{thread::Scope, channel::{Sender, Receiver}};
use anyhow::{Result, Ok};
use crate::{symbols, types::KlineArchive};
use crate::progress;
use crate::klines;

pub fn fetch (interval: String, symbol: String, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>, path: PathBuf, verbosity: Verbosity<InfoLevel>) -> Result<()> {
    let progress = MultiProgress::new();
    let logger = env_logger::Builder::new()
        .filter_level(verbosity.log_level_filter())
        .build();
    
    LogWrapper::new(progress.clone(), logger).try_init().unwrap();

    let (symbol_sender, symbol_receiver) = channel::unbounded();
    let (kline_url_sender, kline_url_receiver) = channel::unbounded();
    let (kline_download_sender, kline_download_receiver) = channel::unbounded();
    let (kline_extract_sender, kline_extract_receiver) = channel::unbounded();

    let _ = scope(|scope| -> Result<()> {
        spawn_fetch_symbols(scope, symbol_sender, symbol, &progress);
        spawn_fetch_kline_urls(scope, symbol_receiver, kline_url_sender, interval, start_date, end_date, &progress);
        spawn_download_klines(scope, kline_url_receiver, kline_download_sender, &path, &progress)?;
        spawn_extract_klines(scope, kline_download_receiver, kline_extract_sender, &progress)?;

        let stats_progress = progress::progress_bar(&progress, "0 kline archives extracted");
        scope.spawn(move |_| {
            let mut count = 0;
            for _ in kline_extract_receiver {
                count += 1;
                stats_progress.set_message(format!("{} kline archives extracted", count));
            }
            stats_progress.finish();
        });
        Ok(())
    }).unwrap();

    Ok(())
}

// BTCUSDT, 1INCHUPUSDT, ...     -> Fetching symbols (1 Worker)
// e.g. https://s3-ap-northeast-1.amazonaws.com/data.binance.vision?delimiter=/&prefix=data/spot/daily/klines/&marker=
fn spawn_fetch_symbols(scope: &Scope<'_>, symbol_sender: Sender<String>, symbol_filter: String, main_progress: &MultiProgress) {
    let progress = progress::progress_bar(main_progress, "Fetching symbols");
    scope.spawn(move |_| {
        symbols::fetch(&symbol_sender, symbol_filter).unwrap();
        progress.finish_with_message("Fetching symbols: done");
        drop(symbol_sender);
    });
}

// Kline URLs -> Fetching kline meta data (50 Workers)
// e.g. https://s3-ap-northeast-1.amazonaws.com/data.binance.vision?delimiter=/&prefix=data/spot/daily/klines/1INCHBTC/1m/
fn spawn_fetch_kline_urls(scope: &Scope<'_>, symbol_receiver: Receiver<String>, kline_url_sender: Sender<String>, interval: String, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>, main_progress: &MultiProgress) {
    let progress = progress::progress_bar(main_progress, "Waiting for symbols...");
    let number_of_workers = 10;

    for _ in 0..number_of_workers {
        let kline_url_sender = kline_url_sender.clone();
        let symbol_receiver = symbol_receiver.clone();
        let progress = progress.clone();
        let interval = interval.clone();

        let _ = scope.spawn(move |_| -> Result<()> {
            for symbol in &symbol_receiver {
                progress.set_message(format!("Fetching kline urls ({} workers, {} symbols in queue)", number_of_workers, symbol_receiver.len()));
                klines::fetch_urls(&symbol, &interval, &kline_url_sender, start_date, end_date)?;
            }
            progress.finish_with_message("Fetching kline urls: done");
            drop(kline_url_sender);
            Ok(())
        });
    }
    drop(kline_url_sender);
}

// Kline download -> Fetching kline data (50 Workers)
// https://data.binance.vision/data/spot/daily/klines/1INCHBTC/1m/1INCHBTC-1m-2020-12-25.zip
fn spawn_download_klines(scope: &Scope<'_>, kline_url_receiver: Receiver<String>, kline_download_sender: Sender<KlineArchive>, data_dir: &Path, main_progress: &MultiProgress) -> Result<()> {
    let progress = progress::progress_bar(main_progress, "Waiting for kline urls...");
    let number_of_workers = 250;

    for _ in 0..number_of_workers {
        let kline_download_sender = kline_download_sender.clone();
        let kline_url_receiver = kline_url_receiver.clone();
        let progress = progress.clone();
        let data_dir = data_dir.to_path_buf();

        let _ = scope.spawn(move |_| -> Result<()> {
            for kline_url in &kline_url_receiver {
                progress.set_message(format!("Downloading kline archives ({} workers, {} urls in queue)", number_of_workers, kline_url_receiver.len()));
                klines::download_klines(&kline_url, &kline_download_sender, &data_dir)?;
            }
            progress.finish_with_message("Downloading klines: done");
            // println!("Drop kline_download_sender");
            drop(kline_download_sender);
            Ok(())
        });
    }
    drop(kline_download_sender);
    Ok(())
}

// Kline extract -> Extract klines (x workers)
fn spawn_extract_klines(scope: &Scope<'_>, kline_download_receiver: Receiver<KlineArchive>, kline_extract_sender: Sender<()>, main_progress: &MultiProgress) -> Result<()> {
    let progress = progress::progress_bar(main_progress, "Waiting for kline archives...");
    let number_of_workers = 5;
    for _ in 0..number_of_workers {
        let kline_download_receiver = kline_download_receiver.clone();
        let kline_extract_sender = kline_extract_sender.clone();
        let progress = progress.clone();

        scope.spawn(move |_| -> Result<()> {
            for download in &kline_download_receiver {
                progress.set_message(format!("Extracting klines ({} workers, {} in queue)", number_of_workers, kline_download_receiver.len()));
                klines::extract_klines(download.temp_file_path, download.target_directory).unwrap();
                kline_extract_sender.send(()).unwrap();
            }
            progress.finish_with_message("Extracting klines: done");
            drop(kline_extract_sender);
            Ok(())
        });
    }
    drop(kline_extract_sender);
    Ok(())
}