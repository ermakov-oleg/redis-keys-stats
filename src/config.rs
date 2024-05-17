use structopt::StructOpt;
use strum::{EnumString, VariantNames};

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Redis DSN
    #[structopt(short = "c", long, default_value = "redis://127.0.0.1:6379/0")]
    pub dsn: String,

    /// Key separators
    #[structopt(long, short, default_value = ".:")]
    pub separators: String,

    /// Depth of the key prefix tree
    #[structopt(long, short, default_value = "5")]
    pub depth: usize,

    /// Minimum count percentage for a prefix to be included in the output
    #[structopt(long, short, default_value = "1")]
    pub min_count_percentage: f32,

    /// Show progress bar
    #[structopt(long, short)]
    pub progress: bool,

    /// Output format
    #[structopt(
        long,
        short = "f",
        possible_values = OutputFormat::VARIANTS,
        case_insensitive = true,
        default_value = "plain",
    )]
    pub output_format: OutputFormat,

    /// Number of keys to scan in one iteration
    #[structopt(long, default_value = "1000")]
    pub scan_size: usize,

    /// Export metrics to Prometheus
    #[structopt(long)]
    pub prometheus: bool,

    /// Prometheus listen address
    #[structopt(long, default_value = "9898")]
    pub prometheus_listen_port: u16,

    /// Collect interval for Prometheus metrics in seconds
    #[structopt(long, default_value = "600")]
    pub collect_interval: u64,

    /// All keys count in db
    #[structopt(skip)]
    pub all_keys_count: usize,
}

#[derive(EnumString, VariantNames, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum OutputFormat {
    /// Plain text output
    Plain,
    /// JSON output
    Json,
    /// No output
    None,
}
