use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct KeyPrefix {
    /// The value of the key prefix.
    pub value: String,
    /// The depth of the key prefix.
    pub depth: usize,
    /// The number of keys with this prefix.
    pub keys_count: usize,
    /// The children of the key prefix.
    pub children: HashMap<String, KeyPrefix>,
}

/// The maximum number of children a key prefix can have.
const MAX_CHILDREN: usize = 10_000;
const OTHER_KEY: &str = "[other]";

impl KeyPrefix {
    pub fn new(prefix: &str, depth: usize, keys_count: usize) -> Self {
        Self {
            value: prefix.to_string(),
            depth,
            keys_count,
            children: HashMap::new(),
        }
    }

    /// Inserts a key path into the key prefix.
    pub fn insert(&mut self, key_path: &[&str]) {
        self.keys_count += 1;

        if key_path.is_empty() {
            return;
        }

        let (key, rest) = key_path.split_at(1);
        let (key, rest): (&str, &[&str]) = if self.children.len() >= MAX_CHILDREN {
            (OTHER_KEY, &[])
        } else {
            (key[0], rest)
        };

        let node = self
            .children
            .entry(key.to_string())
            .or_insert_with(|| KeyPrefix::new(key, self.depth + 1, 0));
        node.insert(rest);
    }

    /// Filter keys based on the minimum count percentage.
    pub fn filter_keys(&mut self, config: &Config) {
        if self.children.is_empty() {
            return;
        }

        let mut other_prefix = self
            .children
            .remove(OTHER_KEY)
            .unwrap_or_else(|| KeyPrefix::new(OTHER_KEY, self.depth + 1, 0));

        self.children.retain(|_key, child| {
            let child_ratio = child.keys_count as f32 / config.all_keys_count as f32 * 100.;
            if child_ratio < config.min_count_percentage {
                other_prefix.keys_count += child.keys_count;
                false
            } else {
                child.filter_keys(config);
                true
            }
        });

        let other_ratio = other_prefix.keys_count as f32 / config.all_keys_count as f32 * 100.;

        if other_ratio > config.min_count_percentage {
            self.children
                .insert(other_prefix.value.clone(), other_prefix);
        }
    }
}
