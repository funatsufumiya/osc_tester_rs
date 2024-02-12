# OSC Tester (Rust)

This is a simple tool to test the OSC communication between a client and a server.

## Build

```bash
$ cargo build --release
```

## Usage

### OSC Receiver

```bash
$ osc-tester server
# Listening on 127.0.0.1:5005...
# [2024-02-12 10:37:42.448582] /hoge 1 2 hoge (type tags: iis)
# [2024-02-12 10:38:41.971990] /hoge 1 2 hoge (type tags: iis)
# [2024-02-12 10:39:00.811072] /hoge 1 2 hoge (type tags: iis)
# [2024-02-12 10:39:05.522840] /hoge 1 2.0 hoge (type tags: ifs)
```

### OSC Sender

```bash
$ osc-tester send /hoge 1 2.0 hoge
# Sending to 127.0.0.1:5005
# [2024-02-12 10:39:05.522620] /hoge 1 2.0 hoge (type tags: ifs)
```

### Sample sender

```bash
$ osc-tester sample
# Sending to 127.0.0.1:5005... (Ctrl+C to quit)
# [2024-02-12 10:45:16.000462] /filter 0.6610950773002804
# [2024-02-12 10:45:17.002817] /filter 0.8154223208829204
# [2024-02-12 10:45:18.004950] /filter 0.37209750414016063
# [2024-02-12 10:45:19.010492] /filter 0.46979363082349024
```