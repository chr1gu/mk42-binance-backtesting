use anyhow::Result;
use chrono::NaiveDate;
use crate::{types::{self, KlineArchive}, date::DateString};
use crossbeam::channel::Sender;
use log::debug;
use regex::Regex;
use serde_xml_rs::from_str;
use std::{io::Read, path::{PathBuf, Path}, env, fs::File};

pub fn fetch_urls (
    symbol: &str,
    interval: &str,
    kline_url_sender: &Sender<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<()> {
    let mut is_fetching = true;
    let mut next_marker: String = String::new();

    while is_fetching {
        let url = format!("https://s3-ap-northeast-1.amazonaws.com/data.binance.vision?delimiter=/&prefix=data/spot/daily/klines/{symbol}/{interval}/&marker={next_marker}");

        // TODO logger
        //println!("Fetching {}", url);
        let mut res = reqwest::blocking::get(&url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let doc: types::KlineResult = from_str(&body).unwrap();
        if let Some(contents) = doc.contents {
            contents.iter().filter(|content| content.key.ends_with(".zip")).for_each(|item| {
                let path = &item.key;
                let regex = Regex::new(r"(?P<date>\d{4}-\d{2}-\d{2})\.zip$").unwrap();
                let Some(captures) = regex.captures(path) else {
                    panic!("{}", format!("Can't parse date: {url}"));
                };

                let date = &captures.name("date").unwrap().as_str().as_date().unwrap();
                if start_date.is_some_and(|start_date| date < &start_date) || end_date.is_some_and(|end_date| date > &end_date) {
                    // ignore urls that are not within the provided start and end dates
                    return;
                }

                let kline_url = format!("https://data.binance.vision/{path}");
                kline_url_sender.send(kline_url).unwrap();
            });
        }

        if let Some(marker) = doc.next_marker {
            next_marker = marker;
        } else {
            is_fetching = false;
        }
    }

    Ok(())
}

pub fn download_klines (
    url: &str,
    kline_download_sender: &Sender<KlineArchive>,
    data_dir: &Path,
) -> Result<()> {
    // Example url: https://data.binance.vision/data/spot/daily/klines/AAVEBUSD/1m/AAVEBUSD-1m-2023-08-07.zip
    let regex = Regex::new(r"/klines/(?P<symbol>\w+)/(?P<interval>\w+)/\w+-\w+-(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\.zip").unwrap();
    let Some(captures) = regex.captures(url) else {
        panic!("{}", format!("Can't parse URL: {url}"));
    };

    let symbol = &captures.name("symbol").unwrap().as_str();
    let interval = &captures.name("interval").unwrap().as_str();
    let year = &captures.name("year").unwrap().as_str();
    let month = &captures.name("month").unwrap().as_str();
    let day = &captures.name("day").unwrap().as_str();

    let file_name = format!("{symbol}-{interval}-{year}-{month}-{day}");
    let target_path = data_dir.join(format!("{year}/{month}/{day}/{file_name}.csv"));

    // Skip files that have been downloaded already
    if target_path.is_file() {
        //println!("Skipped {} because it already exists", url);
        return Ok(());
    }

    let temp_file_path = env::temp_dir().join(format!("{file_name}.zip"));
    let mut temp_file = File::create(&temp_file_path)?;

    reqwest::blocking::get(url)?
        .copy_to(&mut temp_file)
        .unwrap();

    kline_download_sender.send(KlineArchive {
        target_directory: target_path.parent().unwrap().to_path_buf(),
        temp_file_path, 
    }).unwrap();

    Ok(())
}

pub fn extract_klines (
    temp_file_path: PathBuf,
    target_directory: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(temp_file_path)?;
    let mut zip = zip::ZipArchive::new(&zip_file)?;
    zip.extract(&target_directory)?;

    let names: Vec<&str> = zip.file_names().collect();
    debug!("Extracted {:?} into {}", names, target_directory.to_str().unwrap());
    Ok(())
}