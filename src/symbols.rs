use crossbeam::channel::Sender;
use log::debug;
use regex::Regex;
use serde_xml_rs::from_str;
use std::io::Read;

use crate::types;

pub fn fetch(
    symbol_sender: &Sender<String>,
    symbol_filter: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut is_fetching = true;
    let mut next_marker: String = String::new();

    while is_fetching {
        let url = format!("https://s3-ap-northeast-1.amazonaws.com/data.binance.vision?delimiter=/&prefix=data/spot/daily/klines/&marker={next_marker}");
        // TODO: add logger
        //println!("Fetching symbols from {}", url);
        // progress_bar.set_message(format!("Fetching symbols..."));
        let mut res = reqwest::blocking::get(url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let doc: types::SymbolResult = from_str(&body).unwrap();
        if let Some(marker) = doc.next_marker {
            next_marker = marker;
        } else {
            is_fetching = false;
        }

        doc.common_prefixes.iter().for_each(|item| {
            let segments: Vec<&str> = item.prefix.split('/').collect();
            let symbol = segments.get(segments.len() - 2).unwrap();
            let symbol_filter = Regex::new(&symbol_filter).unwrap();

            // discard symbols that do not match
            if symbol_filter.is_match(symbol) {
                symbol_sender.send(symbol.to_string()).unwrap();
                debug!("Symbol loaded: {}", symbol);
            }
        });
    }
    Ok(())
}
