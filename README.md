# Redis keys stats

This is cli tool to get stats of redis keys.

It can be used to get stats about keys counts by prefixes.

## Features

- Show stats of keys by prefixes
- Export metrics to Prometheus
- Show scan progress 

## Example output

```bash
❯ redis-keys-stats -c redis://127.0.0.1:6379/0 --progress --depth 6
[00:00:10] ██████████████████████████████████████████████████████████ 646733/646733 (100%) [ETA: 00:00:00]

Took 10s 508ms 965us
                            Keys Count
ALL ----------------------- 646733 (100.00%)
├─ product ---------------- 246817 (38.16%)
│  └─ quantity ------------ 246817 (100.00%)
│     ├─ backorder -------- 98484 (39.90%)
│     │  └─ [other] ------- 98484 (100.00%)
│     ├─ available -------- 92860 (37.62%)
│     │  └─ [other] ------- 92860 (100.00%)
│     └─ stock ------------ 55473 (22.48%)
│        └─ [other] ------- 55473 (100.00%)
├─ user ------------------- 239334 (37.01%)
│  └─ details ------------- 239334 (100.00%)
│     ├─ account ---------- 146552 (61.23%)
│     │  └─ [other] ------- 146552 (100.00%)
│     └─ profile ---------- 92782 (38.77%)
│        └─ [other] ------- 92782 (100.00%)
└─ order ------------------ 160582 (24.83%)
   └─ status -------------- 160582 (100.00%)
      ├─ shipped ---------- 95965 (59.76%)
      │  └─ tracking ------ 95965 (100.00%)
      │     └─ [other] ---- 95965 (100.00%)
      └─ processing ------- 64617 (40.24%)
         └─ [other] ------- 64617 (100.00%)

```


## Usage

```bash
redis-keys-stats 0.3.0

USAGE:
    redis-keys-stats [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
    -p, --progress      Show progress bar
        --prometheus    Export metrics to Prometheus
    -V, --version       Prints version information

OPTIONS:
        --collect-interval <collect-interval>
            Collect interval for Prometheus metrics in seconds [default: 600]

    -d, --depth <depth>                                      Depth of the key prefix tree [default: 5]
    -c, --dsn <dsn>                                          Redis DSN [default: redis://127.0.0.1:6379/0]
    -m, --min-count-percentage <min-count-percentage>
            Minimum count percentage for a prefix to be included in the output [default: 1]

    -f, --output-format <output-format>
            Output format [default: plain]  [possible values: plain, json, none]

        --prometheus-listen-port <prometheus-listen-port>    Prometheus listen address [default: 9898]
        --scan-size <scan-size>                              Number of keys to scan in one iteration [default: 1000]
    -s, --separators <separators>                            Key separators [default: .:]


```


### Example usage in k8s

```bash
kubectl run -it --image=ghcr.io/ermakov-oleg/redis-keys-stats:latest redis-keys-stats --restart=Never --namespace=default --rm -- -c 'redis://<host>:6379/0' --progress 
```

### Example prometheus metrics

```bash
# HELP redis_key_stats_all_keys Number of all keys.
# TYPE redis_key_stats_all_keys gauge
redis_key_stats_all_keys{dsn="redis://localhost:6379/0"} 1328463
# HELP redis_key_stats_keys_count_by_prefix Number of keys by prefix.
# TYPE redis_key_stats_keys_count_by_prefix gauge
redis_key_stats_keys_count_by_prefix{dsn="redis://localhost:6379/0",prefix="product::quantity::backorder::[other]"} 223036
redis_key_stats_keys_count_by_prefix{dsn="redis://localhost:6379/0",prefix="order::status::shipped::tracking::[other]"} 172526
redis_key_stats_keys_count_by_prefix{dsn="redis://localhost:6379/0",prefix="order::status::processing::[other]"} 151672
redis_key_stats_keys_count_by_prefix{dsn="redis://localhost:6379/0",prefix="user::details::profile::[other]"} 117046
redis_key_stats_keys_count_by_prefix{dsn="redis://localhost:6379/0",prefix="product::quantity::available::[other]"} 278080
redis_key_stats_keys_count_by_prefix{dsn="redis://localhost:6379/0",prefix="product::quantity::stock::[other]"} 257569
redis_key_stats_keys_count_by_prefix{dsn="redis://localhost:6379/0",prefix="user::details::account::[other]"} 128534
```
