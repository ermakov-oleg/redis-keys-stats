use crate::config::Config;
use crate::key_prefix::KeyPrefix;
use crate::stats::Result;
use crate::utils::get_masked_dsn;
use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};
use prometheus_client::registry::Registry;
use std::io::Cursor;
use std::thread;
use std::thread::JoinHandle;
use structopt::lazy_static::lazy_static;

lazy_static! {
    static ref KEYS_COUNT_BY_PREFIX: Family<KeyPrefixLabel, Gauge> =
        Family::<KeyPrefixLabel, Gauge>::default();
    static ref ALL_KEYS_COUNT: Family<DsnLabel, Gauge> = Family::<DsnLabel, Gauge>::default();
    static ref SCAN_DURATION: Family<DsnLabel, Histogram> =
        Family::<DsnLabel, Histogram>::new_with_constructor(|| Histogram::new(
            exponential_buckets(10.0, 1.5, 20)
        ));
    static ref REGISTRY: Registry = make_registry();
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct DsnLabel {
    dsn: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct KeyPrefixLabel {
    dsn: String,
    prefix: String,
}

pub fn start_metrics_server(port: u16) -> JoinHandle<()> {
    let addr = format!("0.0.0.0:{port}");
    println!("Serving metrics on {}", addr);
    let server = tiny_http::Server::http(addr).unwrap();

    thread::spawn(move || {
        for rq in server.incoming_requests() {
            let _ = rq.respond(metrics_handler());
        }
    })
}

pub fn update_metrics(config: &Config, result: &Result) {
    KEYS_COUNT_BY_PREFIX.clear();
    let dsn = get_masked_dsn(&config.dsn);

    let dsn_label = DsnLabel { dsn: dsn.clone() };

    SCAN_DURATION
        .get_or_create(&dsn_label)
        .observe(result.took.as_secs_f64());
    ALL_KEYS_COUNT
        .get_or_create(&dsn_label)
        .set(result.root_prefix.keys_count as i64);

    fn iter_metrics(prefix: &KeyPrefix, current_prefix: &str, dsn: &str) {
        let current_prefix = if current_prefix.is_empty() {
            prefix.value.clone()
        } else {
            format!("{}::{}", current_prefix, prefix.value)
        };
        if prefix.children.is_empty() {
            let label = KeyPrefixLabel {
                dsn: dsn.to_string(),
                prefix: current_prefix,
            };

            KEYS_COUNT_BY_PREFIX
                .get_or_create(&label)
                .set(prefix.keys_count as i64);
        } else {
            for child in prefix.children.values() {
                iter_metrics(child, &current_prefix, dsn);
            }
        };
    }
    iter_metrics(&result.root_prefix, "", &dsn);
}

fn make_registry() -> Registry {
    let mut metrics_registry = Registry::with_prefix("redis_key_stats");

    metrics_registry.register("all_keys", "Number of all keys", ALL_KEYS_COUNT.clone());
    metrics_registry.register(
        "keys_count_by_prefix",
        "Number of keys by prefix",
        KEYS_COUNT_BY_PREFIX.clone(),
    );
    metrics_registry.register(
        "scan_duration",
        "Duration of redis scan",
        SCAN_DURATION.clone(),
    );

    metrics_registry
}

fn metrics_handler() -> tiny_http::Response<Cursor<Vec<u8>>> {
    let mut encoded = String::new();
    encode(&mut encoded, &REGISTRY).expect("encoding metrics");
    tiny_http::Response::new(
        tiny_http::StatusCode(200),
        vec![tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap()],
        Cursor::new(encoded.into_bytes()),
        None,
        None,
    )
}
