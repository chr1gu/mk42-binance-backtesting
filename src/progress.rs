use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub fn progress_bar(progress: &MultiProgress, message: &'static str) -> ProgressBar {
    let spinner_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_msg}").unwrap();
    let progress_bar = progress.add(ProgressBar::new(1));
    progress_bar.set_style(spinner_style.clone());
    progress_bar.enable_steady_tick(Duration::from_millis(100));
    progress_bar.set_message(message);
    progress_bar
}