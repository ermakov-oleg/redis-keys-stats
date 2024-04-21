# Redis keys stats

This is cli tool to get stats of redis keys.

It can be used to get stats about keys counts by prefixes.

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
│     │  └─ [other] ------- 10000 (10.15%)
│     ├─ available -------- 92860 (37.62%)
│     │  └─ [other] ------- 10000 (10.77%)
│     └─ stock ------------ 55473 (22.48%)
│        └─ [other] ------- 10000 (18.03%)
├─ user ------------------- 239334 (37.01%)
│  └─ details ------------- 239334 (100.00%)
│     ├─ account ---------- 146552 (61.23%)
│     │  └─ [other] ------- 10000 (6.82%)
│     └─ profile ---------- 92782 (38.77%)
│        └─ [other] ------- 10000 (10.78%)
└─ order ------------------ 160582 (24.83%)
   └─ status -------------- 160582 (100.00%)
      ├─ shipped ---------- 95965 (59.76%)
      │  └─ tracking ------ 95965 (100.00%)
      │     └─ [other] ---- 10000 (10.42%)
      └─ processing ------- 64617 (40.24%)
         └─ [other] ------- 10000 (15.48%)
```


## Usage

```bash
redis-keys-stats 0.1.0

USAGE:
    redis-keys-stats [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -p, --progress    Show progress bar
    -V, --version     Prints version information

OPTIONS:
    -d, --depth <depth>                                  Depth of the key prefix tree [default: 5]
    -c, --dsn <dsn>                                      Redis DSN [default: redis://127.0.0.1:6379/0]
    -m, --min-count-percentage <min-count-percentage>
            Minimum count percentage for a prefix to be included in the output [default: 1]

    -f, --output-format <output-format>                  Output format [default: plain]  [possible values: plain, json]
        --scan-size <scan-size>                          Number of keys to scan in one iteration [default: 1000]
    -s, --separators <separators>                        Key separators [default: .:]

```
