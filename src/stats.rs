use std::cmp::min;
use std::time::{Duration, SystemTime};

use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Config;
use crate::key_prefix::KeyPrefix;

/// The result of the analysis
pub struct Result {
    /// The root prefix of the analyzed keys
    pub root_prefix: KeyPrefix,
    /// The time it took to analyze the keys
    pub took: Duration,
}

/// Run the analysis
pub fn run(config: &mut Config) -> Result {
    let mut root_prefix = KeyPrefix::new("", 0, 0);

    let now = SystemTime::now();
    analyze_count(config, &mut root_prefix);
    let took = now.elapsed().unwrap();

    Result { root_prefix, took }
}

/// Analyze the keys count
fn analyze_count(config: &mut Config, prefix: &mut KeyPrefix) {
    let mut redis_client = connect_redis(&config.dsn);
    let all_keys_count: usize = redis::cmd("DBSIZE")
        .query(&mut redis_client)
        .expect("getting db size");

    config.all_keys_count = all_keys_count;

    let bar = if config.progress {
        ProgressBar::new(config.all_keys_count as u64)
    } else {
        ProgressBar::hidden()
    };

    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}] {wide_bar} {pos}/{len} ({percent}%) [ETA: {eta_precise}]",
            )
            .expect("Failed to set progress bar style"),
    );

    let scan_command = redis::cmd("SCAN")
        .cursor_arg(0)
        .arg("COUNT")
        .arg(config.scan_size)
        .clone();

    let iter: redis::Iter<String> = scan_command.iter(&mut redis_client).expect("running scan");

    for (i, key) in iter.enumerate() {
        if i % 10_000 == 0 && i > 0 {
            bar.inc(10_000);
        }

        let key_items: Vec<_> = key.split(|e| config.separators.contains(e)).collect();
        prefix.insert(&key_items[..min(key_items.len(), config.depth)]);
    }

    bar.finish();

    prefix.filter_keys(config)
}

/// Connect to redis
fn connect_redis(dsn: &str) -> redis::Connection {
    let client =
        redis::Client::open(dsn).unwrap_or_else(|_| panic!("Failed to connect to redis ({})", dsn));
    client
        .get_connection()
        .unwrap_or_else(|_| panic!("Failed to connect to redis ({})", dsn))
}
