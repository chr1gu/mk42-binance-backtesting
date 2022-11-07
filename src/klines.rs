use crate::types;
use chrono::{Duration, NaiveDate};
use colored::Colorize;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use serde_xml_rs::from_str;
use std::{io::Read, ops::Sub, path::PathBuf};

pub fn get_klines(
    symbol: &str,
    interval: &str,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
    path: &PathBuf,
    force: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = format!(
        "https://s3-ap-northeast-1.amazonaws.com/data.binance.vision?delimiter=/&prefix=data/spot/daily/klines/{symbol}/{interval}/",
    );

    if let Some(start_date) = start_date {
        let previous_day = start_date.sub(Duration::days(1));
        let date_day = previous_day.format("%d").to_string();
        let date_month = previous_day.format("%m").to_string();
        let date_year = previous_day.format("%Y").to_string();
        url = format!(
            "{url}&marker=data/spot/daily/klines/{symbol}/{interval}/{symbol}-{interval}-{date_year}-{date_month}-{date_day}.zip"
        )
    }

    println!("Fetching {}", url);
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let doc: types::KlineResult = from_str(&body).unwrap();

    // skip if there is no data.
    if doc.contents.is_none() {
        return Ok(());
    }

    doc.contents
        .unwrap()
        .par_iter()
        .filter(|content| content.key.ends_with(".zip"))
        .for_each(|content| {
            get_archive(
                symbol,
                interval,
                &format!("https://data.binance.vision/{}", content.key),
                path,
                start_date,
                end_date,
                force,
            );
        });

    // if there is more data to fetch, check if end_date is <= next markers date
    if let Some(next_marker) = doc.next_marker {
        // e.g. data/spot/daily/klines/1INCHBTC/1m/1INCHBTC-1m-2022-07-13.zip.CHECKSUM
        let date_regex = Regex::new(r"(?:([0-9]{4}-[0-9]{2}-[0-9]{2}))").unwrap();
        let date_str = date_regex
            .captures(&next_marker)
            .unwrap()
            .get(1)
            .map_or("", |m| m.as_str());
        let next_start_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        if end_date
            .map(|end_date| end_date.ge(&next_start_date))
            .unwrap_or(true)
        {
            return get_klines(
                symbol,
                interval,
                &Some(next_start_date),
                end_date,
                path,
                force,
            );
        }
    }

    Ok(())
}

fn get_archive(
    symbol: &str,
    interval: &str,
    url: &str,
    path: &PathBuf,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
    force: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut segments: Vec<&str> = url.split("/").collect();
    let date = segments
        .pop()
        .unwrap()
        .replace(&format!("{}-{}-", symbol, interval), "")
        .replace(&".zip", "");

    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();

    if let Some(start_date) = start_date {
        if parsed_date.lt(start_date) {
            return Ok(());
        }
    }

    if let Some(end_date) = end_date {
        if parsed_date.gt(end_date) {
            return Ok(());
        }
    }

    // path with folder structure
    let path = path
        .join(parsed_date.format("%Y").to_string())
        .join(parsed_date.format("%m").to_string())
        .join(parsed_date.format("%d").to_string());
    let file_name = format!("{}-{}-{}.csv", symbol, interval, date);

    if !force && path.join(&file_name).is_file() {
        println!("{} already exists, skip.", &file_name);
        return Ok(());
    }

    let mut tmp = tempfile::tempfile()?;
    reqwest::blocking::get(url)?
        .copy_to(&mut tmp)
        .expect(&format!("Could not unpack {url}.").red());

    let mut zip = zip::ZipArchive::new(&tmp)?;
    // let file = zip.by_index(0)?;
    // let file_name = file.name().to_string();

    // let mut reader = csv::Reader::from_reader(file);
    // let rows = reader.records().count() + 1;

    zip.extract(path)?;

    // verbose only: show all extracted files
    // println!("extracted {:#?}, number of rows: {}", file_name, rows);
    println!("extracted {}", url);
    Ok(())
}
