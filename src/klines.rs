use std::{
    io::Read,
    path::{Path, PathBuf},
};

use crate::{cli, types};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde_xml_rs::from_str;

pub fn get_klines(symbol: &str, args: &cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
    let interval = &args.interval;

    let url = format!(
        "https://s3-ap-northeast-1.amazonaws.com/data.binance.vision?delimiter=/&prefix=data/spot/daily/klines/{}/{}/",
        symbol, interval
    );

    println!("Fetching {}", url);
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let doc: types::KlineResult = from_str(&body).unwrap();

    // TODO: run through all markers
    doc.contents
        .par_iter()
        .filter(|content| content.key.ends_with(".zip"))
        .for_each(|content| {
            get_archive(
                symbol,
                interval,
                &format!("https://data.binance.vision/{}", content.key),
                &args.path,
            );
        });

    Ok(())
}

fn get_archive(
    symbol: &str,
    interval: &str,
    url: &str,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // https://data.binance.vision/data/spot/daily/klines/BNBUSDT/1s/BNBUSDT-1s-2018-03-29.zip
    // https://data.binance.vision/data/spot/daily/klines/BNBUSDT/1m/BNBUSDT-1m-2021-03-01.zip
    // https://data.binance.vision/data/spot/daily/klines/BNBUSDT/1s/BNBUSDT-1s-2021-03-01.zip
    let mut tmp = tempfile::tempfile()?;
    reqwest::blocking::get(url)?.copy_to(&mut tmp).unwrap();

    let mut zip = zip::ZipArchive::new(&tmp)?;
    // let file = zip.by_index(0)?;
    // let file_name = file.name().to_string();

    // let mut reader = csv::Reader::from_reader(file);
    // let rows = reader.records().count() + 1;

    let mut segments: Vec<&str> = url.split("/").collect();
    let folder = segments
        .pop()
        .unwrap()
        .replace(&format!("{}-{}-", symbol, interval), "")
        .replace(&".zip", "");

    // extract
    let path = Path::join(path, folder);
    zip.extract(path)?;

    // verbose only: show all extracted files
    // println!("extracted {:#?}, number of rows: {}", file_name, rows);
    println!("extracted {}", url);
    Ok(())
}
