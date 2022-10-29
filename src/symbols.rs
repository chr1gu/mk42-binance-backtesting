use crate::types;
use serde_xml_rs::from_str;
use std::io::Read;
use urlencoding::encode;

pub fn get_symbols() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut is_fetching = true;
    let mut symbols: Vec<String> = vec![];
    let mut next_marker: String = String::new();

    while is_fetching {
        let url = format!("https://s3-ap-northeast-1.amazonaws.com/data.binance.vision?delimiter=/&prefix=data/spot/daily/klines/&marker={}", next_marker);
        println!("Fetching symbols from {}", url);
        let mut res = reqwest::blocking::get(url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let doc: types::SymbolResult = from_str(&body).unwrap();
        if let Some(marker) = doc.next_marker {
            next_marker = encode(&marker).to_string();
        } else {
            is_fetching = false;
        }

        doc.common_prefixes.iter().for_each(|item| {
            let segments: Vec<&str> = item.prefix.split("/").collect();
            let symbol = segments.get(segments.len() - 2).unwrap();
            symbols.push(symbol.to_string());
        })
    }
    Ok(symbols)
}
