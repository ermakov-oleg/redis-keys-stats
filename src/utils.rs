use redis::ConnectionInfo;

/// Get Redis DSN host
pub fn get_dsn_host(dsn: &str) -> String {
    let conn_info: ConnectionInfo = dsn.parse().expect("parsing redis url");
    return format!("{}/{}", conn_info.addr, conn_info.redis.db);
}
