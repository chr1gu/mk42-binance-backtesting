# Binance Public Data CLI

Easily download [Binance Public Data](https://github.com/binance/binance-public-data) 💪

## Getting started

1. Install and build `binance-get` from source:

```
cargo install --git https://github.com/chr1gu/binance-public-data-cli
```

2. Example usage:

```
mkdir ./binance-market-data
binance-get --symbol ^BTC --interval 1m ./binance-market-data
```

## Development

Run `cargo r` to compile and run the package with default settings.
Run e.g. `cargo r --release -- --start-date 2021-03-01 --end-date 2022-01-01 --symbol USDT$ --interval 1m ./target` with custom settings.

### New release

```
git tag -a v0.0.1
git push --tags
```
