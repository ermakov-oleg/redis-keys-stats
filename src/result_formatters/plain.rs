use std::cmp::max;

use humantime::format_duration;

use crate::key_prefix::KeyPrefix;
use crate::stats::Result;

pub fn call(result: &Result) {
    let key_column_width = calculate_key_column_width(&result.root_prefix);

    println!("Took {}", format_duration(result.took));
    println!("{:indent$} Keys Count", "", indent = key_column_width,);

    print_tree(
        &result.root_prefix,
        &result.root_prefix,
        "".to_string(),
        true,
        false,
        key_column_width,
    );
}

/// Print the tree of key prefixes
fn print_tree(
    node: &KeyPrefix,
    parent_node: &KeyPrefix,
    prefix: String,
    root: bool,
    last: bool,
    key_column_width: usize,
) {
    let prefix_current = if last { "└─ " } else { "├─ " };

    let (leaf, info) = if root {
        let leaf = format!("{}{} ", "ALL", node.value);
        let info = display_count(node, parent_node);
        (leaf, info)
    } else {
        let leaf_prefix = format!("{}{}", prefix, prefix_current);
        let leaf = format!("{}{} ", leaf_prefix, node.value);
        let info = display_count(node, parent_node);
        (leaf, info)
    };

    println!(
        "{leaf:-<width$}{info}",
        leaf = leaf,
        width = key_column_width,
        info = info,
    );

    let prefix_child = if root {
        ""
    } else if last {
        "   "
    } else {
        "│  "
    };
    let prefix = prefix + prefix_child;

    if !node.children.is_empty() {
        let last_child = node.children.len() - 1;

        let mut sorted_nodes = node.children.values().collect::<Vec<_>>();
        sorted_nodes.sort_by(|a, b| b.keys_count.cmp(&a.keys_count));

        for (i, child) in sorted_nodes.iter().enumerate() {
            print_tree(
                child,
                node,
                prefix.to_string(),
                false,
                i == last_child,
                key_column_width,
            );
        }
    }
}

/// Display the count of keys
fn display_count(prefix: &KeyPrefix, parent_prefix: &KeyPrefix) -> String {
    format!(
        " {count} ({percentage:.2}%) ",
        count = prefix.keys_count,
        percentage = prefix.keys_count as f32 / parent_prefix.keys_count as f32 * 100.,
    )
}

/// Calculate the width of the key column
fn calculate_key_column_width(root_prefix: &KeyPrefix) -> usize {
    let padding = 5;
    biggest_key_length(root_prefix) + padding
}

/// Calculate maximum key length
fn biggest_key_length(prefix: &KeyPrefix) -> usize {
    let display_value = prefix.value.to_string();
    let length = display_value.len() + prefix.depth * 3;

    prefix
        .children
        .iter()
        .fold(length, |acc, child| max(acc, biggest_key_length(child.1)))
}
