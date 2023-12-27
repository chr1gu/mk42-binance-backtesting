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

# Visualize

## Grafana

docker run --rm -it -p 3000:3000 --name=grafana \
 --user "$(id -u)" \
 -v "$PWD/grafana/data:/var/lib/grafana" \
 -v "$PWD/data:/data" \
 grafana/grafana-oss

> Open http://localhost:3000/
> admin / admin

## InfluxDB

docker run --rm -it -p 8083:8083 -p 8086:8086 \
 --user "$(id -u)" \
 -v "$PWD/influxdb/data:/var/lib/influxdb2" \
 influxdb

> Open http://localhost:8086/
> admin / admin123

## Development

- Update toolchain: [Install Rust](https://www.rust-lang.org/tools/install) or update your installation with `rustup update`.
- Build and fetch data: `cargo r --release -- fetch --start-date 2021-03-01 --end-date 2022-01-01 --symbol USDT$ --interval 1m ./data`
- Build and test with data: `cargo r --release -- test --start-date 2021-03-01 --end-date 2022-01-01 --symbol "BTCUSDT|XRPUSDT" ./data --verbose`
- Build and test with multiple variants: `cargo r --release -- test-variants --start-date 2021-01-01 --end-date 2022-01-01 --symbol USDT$ ./data`

### Linting

Run `cargo clippy` or `cargo clippy --fix --bin "mk42-binance-backtesting"` to find and fix obvious issues.

### New release

```
git tag -a v0.0.1
git push --tags
```

# Performance

## Highscores

`cargo r --release -- test-variants --start-date 2021-01-01 --end-date 2022-01-01 --symbol USDT$ ./data`

> Average performance from 2021-01-01 to 2022-01-01 with 365 variations: 3069%
> Finished after 3221.323858917s

`cargo r --release -- test-variants --start-date 2022-01-01 --end-date 2023-01-01 --symbol USDT$ ./data`

> Average performance from 2022-01-01 to 2023-01-01 with 365 variations: -2758%
> Finished after 3912.586872958s

## Difficult scenarios

`cargo r --release -- test-variants --start-date 2021-05-01 --end-date 2021-06-01 --symbol USDT$ ./data`

> Average performance from 2021-05-01 to 2021-06-01 with 31 variations: -5553%

`cargo r --release -- test-variants --start-date 2021-06-01 --end-date 2021-07-01 --symbol USDT$ ./data`

> Average performance from 2021-06-01 to 2021-07-01 with 30 variations: -3525%

`cargo r --release -- test-variants --start-date 2021-09-01 --end-date 2021-10-01 --symbol USDT$ ./data`

> Average performance from 2021-09-01 to 2021-10-01 with 30 variations: -1689%

`cargo r --release -- test-variants --start-date 2021-11-01 --end-date 2021-12-01 --symbol USDT$ ./data`

> Average performance from 2021-11-01 to 2021-12-01 with 30 variations: -739%

`cargo r --release -- test-variants --start-date 2021-01-01 --end-date 2022-01-01 --symbol USDT$ ./data`

> Average performance from 2021-01-01 to 2022-01-01 with 365 variations: -1465%
> Finished after 3897.686776459s

`cargo r --release -- test-variants --start-date 2023-04-01 --end-date 2023-06-01 --symbol USDT$ ./data`

> Average performance from 2023-04-01 to 2023-06-01 with 61 variations: -1494%
> Performance from 2023-04-01 to 2023-06-01: -2526%, trades: 885
> Finished after 52.31117825s
