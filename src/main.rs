mod config;
mod key_prefix;
mod prometheus;
mod result_formatters;
mod stats;
mod utils;

use crate::config::Config;
use structopt::StructOpt;

/// Analyze the keys in a Redis database
fn main() {
    let mut config = Config::from_args();
    if config.prometheus {
        prometheus::start_metrics_server(config.prometheus_listen_port);
    }
    loop {
        let result = stats::run(&mut config);
        result_formatters::call(&config, &result);
        if !config.prometheus {
            break;
        }
        prometheus::update_metrics(&config, &result);
        std::thread::sleep(std::time::Duration::from_secs(config.collect_interval));
    }
}
