use clap::{Parser, Subcommand, Args};
use clap_verbosity_flag::{Verbosity, InfoLevel};


#[derive(Debug, Parser)]
#[command(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Print version info and exit
    #[arg(short('V'), long, default_value_t = false)]
    pub version: bool,

    /// Print available symbols and exit
    #[arg(short, long, default_value_t = false)]
    pub list_symbols: bool,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Fetch {
        /// The output directory to store the files
        path: std::path::PathBuf,

        /// The interval
        #[arg(short, long, default_value_t = format!("1m"))]
        interval: String,

        #[command(flatten)]
        shared_args: SharedArguments,
    },

    #[command(arg_required_else_help = true)]
    Run {
        #[command(flatten)]
        shared_args: SharedArguments,
    }
}

#[derive(Args, Debug, Clone)]
pub struct SharedArguments {
    /// The symbol name or Regex filter
    #[arg(short, long, default_value_t = format!(".*"))]
    pub symbol: String,

    /// Optional: start date (format: YYYY-MM-DD)
    #[arg(long)]
    pub start_date: Option<String>,

    /// Optional: end date (format: YYYY-MM-DD)
    #[arg(long)]
    pub end_date: Option<String>,
}