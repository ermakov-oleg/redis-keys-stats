mod config;
mod key_prefix;
mod result_formatters;
mod stats;

use crate::config::Config;
use structopt::StructOpt;

/// Analyze the keys in a Redis database
fn main() {
    let mut config = Config::from_args();
    let result = stats::run(&mut config);
    result_formatters::call(&config, &result);
}
