use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The output directory to store the files
    pub path: std::path::PathBuf,

    /// The interval
    #[arg(short, long, default_value_t = format!("1s"))]
    pub interval: String,

    /// The symbol name or Regex filter
    #[arg(short, long, default_value_t = format!(".*"))]
    pub symbol: String,

    // Print available symbols and exit
    #[arg(short, long, default_value_t = false)]
    pub list_symbols: bool,

    /// Print version info and exit
    #[arg(short('V'), long, default_value_t = false)]
    pub version: bool,
}
