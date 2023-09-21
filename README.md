# MK42 Binance Backtesting Tools

## Fetch: download Binance public data

Use `mk42-binance-backtesting fetch` to easily download [Binance public data](https://github.com/binance/binance-public-data) 💪

## Getting started

1. Install and build `mk42-binance-backtesting` from source:

```
cargo install --git https://github.com/chr1gu/mk42-binance-backtesting
```

2. Example usage:

```
mk42-binance-backtesting fetch --symbol ^BTC --interval 1m ./data
```

## Development

[Install Rust](https://www.rust-lang.org/tools/install) or update your installation: `rustup update` then
run e.g. `cargo r --release -- fetch --start-date 2021-03-01 --end-date 2022-01-01 --symbol USDT$ --interval 1m ./data` with custom settings.

### Linting

Run `cargo clippy`

### New release

```
git tag -a v0.0.1
git push --tags
```

## TODOs

- Use anyhow or eyre
