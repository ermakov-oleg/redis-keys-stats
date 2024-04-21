pub mod json;
pub mod plain;

use crate::config::{Config, OutputFormat};
use crate::stats::Result;

/// Call the appropriate formatter based on the output format
pub fn call(config: &Config, result: &Result) {
    let formatter = match config.output_format {
        OutputFormat::Plain => plain::call,
        OutputFormat::Json => json::call,
    };
    formatter(result);
}
