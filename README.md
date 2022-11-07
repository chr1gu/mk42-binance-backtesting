# Binance Public Data CLI

Easily download [Binance Public Data](https://github.com/binance/binance-public-data) 💪

## Development

Run `cargo r` to compile and run the package with default settings.
Run e.g. `cargo r --release -- --start-date 2021-03-01 --end-date 2022-01-01 --symbol USDT$ --interval 1m ./target` with custom settings.

### New release

```
git tag -a v0.0.1
git push --tags
```
