use crate::klines;
use crate::progress;
use crate::{symbols, types::KlineArchive};
use anyhow::{Ok, Result};
use chrono::NaiveDate;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use crossbeam::{
    channel::{self},
    scope,
};
use crossbeam::{
    channel::{Receiver, Sender},
    thread::Scope,
};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use regex::Regex;
use std::path::{Path, PathBuf};

pub fn visualize(
    symbol_filter: &Regex,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    data_dir: PathBuf,
    progress: &MultiProgress,
) -> Result<()> {
    Ok(())
}
