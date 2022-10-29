use clap::Parser;
use regex::Regex;
mod cli;
mod klines;
mod symbols;
mod types;

fn main() {
    let args = cli::Cli::parse();
    if args.version {
        return println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    let all_symbols = symbols::get_symbols().unwrap();
    if args.list_symbols {
        return println!("{}", all_symbols.join(", "));
    }

    let symbol_filter = Regex::new(&args.symbol).unwrap();
    let symbols: Vec<String> = all_symbols
        .iter()
        .filter(|s| symbol_filter.is_match(s))
        .map(|s| s.to_owned())
        .collect();

    println!("{} symbols found: {}", symbols.len(), symbols.join(", "));
    symbols.iter().for_each(|symbol| {
        klines::get_klines(symbol, &args);
    });
    println!("Done!");
}
