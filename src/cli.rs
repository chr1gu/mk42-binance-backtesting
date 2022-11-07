use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The output directory to store the files
    pub path: std::path::PathBuf,

    /// The interval
    #[arg(short, long, default_value_t = format!("1m"))]
    pub interval: String,

    /// The symbol name or Regex filter
    #[arg(short, long, default_value_t = format!(".*"))]
    pub symbol: String,

    /// Force override existing files
    #[arg(short, long, default_value_t = false)]
    pub force: bool,

    /// Optional: start date (format: YYYY-MM-DD)
    #[arg(long)]
    pub start_date: Option<String>,

    /// Optional: end date (format: YYYY-MM-DD)
    #[arg(long)]
    pub end_date: Option<String>,

    /// Print available symbols and exit
    #[arg(short, long, default_value_t = false)]
    pub list_symbols: bool,

    /// Continue fetching data if a request fails for some reason
    #[arg(long, default_value_t = false)]
    pub continue_on_error: bool,

    /// Print version info and exit
    #[arg(short('V'), long, default_value_t = false)]
    pub version: bool,
}
