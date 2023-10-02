use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

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

        /// The symbol name or Regex filter
        #[arg(short, long, default_value_t = format!(".*"))]
        symbol: String,

        /// The interval
        #[arg(short, long, default_value_t = format!("1m"))]
        interval: String,

        /// Optional: start date (format: YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,

        /// Optional: end date (format: YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
    },

    #[command(arg_required_else_help = true)]
    Test {
        /// The symbol name or Regex filter
        #[arg(short, long, default_value_t = format!(".*"))]
        symbol: String,

        /// Start date (format: YYYY-MM-DD)
        #[arg(long)]
        start_date: String,

        /// End date (format: YYYY-MM-DD)
        #[arg(long)]
        end_date: String,

        /// The output directory to store the files
        path: std::path::PathBuf,
    },

    #[command(arg_required_else_help = true)]
    TestVariants {
        /// The symbol name or Regex filter
        #[arg(short, long, default_value_t = format!(".*"))]
        symbol: String,

        /// Start date (format: YYYY-MM-DD)
        #[arg(long)]
        start_date: String,

        /// End date (format: YYYY-MM-DD)
        #[arg(long)]
        end_date: String,

        /// The output directory to store the files
        path: std::path::PathBuf,
    },
}
