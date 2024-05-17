/// Get Redis DSN with password masked
pub fn get_masked_dsn(dsn: &str) -> String {
    let mut parts: Vec<&str> = dsn.split('@').collect();
    if parts.len() == 2 {
        parts[0] = "redis://***:***@";
    }
    parts.join("")
}
