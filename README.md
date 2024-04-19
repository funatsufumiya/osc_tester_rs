# OSC Tester (Rust)

[![Crates.io](https://img.shields.io/crates/v/osc-tester)](https://crates.io/crates/osc-tester)
[![Docs.rs](https://docs.rs/osc-tester/badge.svg)](https://docs.rs/osc-tester)
[![License](https://img.shields.io/crates/l/osc-tester)](LICENSE)

This is a simple tool to test the OSC communication between a client and a server.

## Install

- from crates.io

```bash
$ cargo install osc-tester
```

- from source

```bash
$ cargo install --path .
```

## Usage

### OSC Receiver

(Check `osc-tester receive -h` for options)

```bash
$ osc-tester receive
# Listening on 127.0.0.1:5005...
# [2024-02-12 10:37:42.448582] /hoge 1 2 hoge (type tags: iis)
# [2024-02-12 10:38:41.971990] /hoge 1 2 hoge (type tags: iis)
# [2024-02-12 10:39:00.811072] /hoge 1 2 hoge (type tags: iis)
# [2024-02-12 10:39:05.522840] /hoge 1 2.0 hoge (type tags: ifs)
```

### OSC Sender

(Check `osc-tester send -h` for options)

```bash
$ osc-tester send /hoge 1 2.0 hoge
# Sending to 127.0.0.1:5005
# [2024-02-12 10:39:05.522620] /hoge 1 2.0 hoge (type tags: ifs)
```

### Sample sender

(Check `osc-tester sample -h` for options)

```bash
$ osc-tester sample
# Sending to 127.0.0.1:5005... (Ctrl+C to quit)
# [2024-02-12 10:45:16.000462] /filter 0.6610950773002804
# [2024-02-12 10:45:17.002817] /filter 0.8154223208829204
# [2024-02-12 10:45:18.004950] /filter 0.37209750414016063
# [2024-02-12 10:45:19.010492] /filter 0.46979363082349024
```

## How to build

```bash
$ cargo build --release

# or just try $ cargo run -- server
```