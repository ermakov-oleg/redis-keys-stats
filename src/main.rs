mod stats;
mod config;
mod key_prefix;
mod result_formatters;

use structopt::StructOpt;
use crate::config::Config;

/// Analyze the keys in a Redis database
fn main() {
    let mut config = Config::from_args();
    let result = stats::run(&mut config);
    result_formatters::call(&config, &result);
}
