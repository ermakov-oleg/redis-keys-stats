use serde_json;

use crate::stats::Result;

pub fn call(result: &Result) {
    let json = serde_json::to_string(&result.root_prefix).unwrap();
    println!("{}", json);
}
