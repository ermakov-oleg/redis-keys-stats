pub mod json;
pub mod plain;

use crate::stats::Result;
use crate::config::{Config, OutputFormat};

/// Call the appropriate formatter based on the output format
pub fn call(config: &Config, result: &Result) {
    let formatter = match config.output_format {
        OutputFormat::Plain => plain::call,
        OutputFormat::Json => json::call,
    };
    formatter(result);
}
