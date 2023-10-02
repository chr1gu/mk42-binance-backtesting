use std::fmt;

use crate::klines::Kline;
use anyhow::Ok;
use chrono::NaiveDateTime;
use colored::Colorize;
use log::debug;
use ta::{
    indicators::{
        MovingAverageConvergenceDivergence, OnBalanceVolume, RelativeStrengthIndex,
        SimpleMovingAverage,
    },
    Next,
};

pub struct TradingStatistics {
    pub performance: f64,
    pub total_fee: f64,
    pub total_buys: i32,
    pub total_sells: i32,
    pub total_profitable_sells: i32,
    pub total_stoploss_sells: i32,
    pub updates: i32,
}

pub struct SymbolInfo {
    pub name: String,
}

pub struct TradingSignal {
    pub symbol: SymbolInfo,
    pub stats: TradingStatistics,
    pub sma9: SimpleMovingAverage,
    pub sma26: SimpleMovingAverage,
    pub sma50: SimpleMovingAverage,
    pub sma200: SimpleMovingAverage,
    pub sma201: SimpleMovingAverage,
    pub sma1440: SimpleMovingAverage,
    pub sma10080: SimpleMovingAverage,
    pub macd: MovingAverageConvergenceDivergence,
    pub obv: OnBalanceVolume,
    pub rsi: RelativeStrengthIndex,
    pub current_buy_price: Option<f64>,
    pub latest_buy_timestamp: Option<NaiveDateTime>,
    pub latest_sell_timestamp: Option<NaiveDateTime>,
    pub latest_close: Option<f64>,
}

impl fmt::Display for TradingSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = self.symbol.name.yellow();
        let fees = self.stats.total_fee;
        let sells = self.stats.total_sells;
        let profitable_sells = self.stats.total_profitable_sells;
        let performance = if self.stats.performance > 0.0 {
            self.stats.performance.to_string().green()
        } else if self.stats.performance < 0.0 {
            self.stats.performance.to_string().red()
        } else {
            "n/A".white()
        };
        write!(f, "{symbol}'s performance: {performance}, fees: {fees}, profitable trades: {profitable_sells}/{sells}")
    }
}

impl TradingSignal {
    pub fn new(symbol: String) -> TradingSignal {
        TradingSignal {
            symbol: SymbolInfo { name: symbol },
            stats: TradingStatistics {
                performance: 0.0,
                updates: 0,
                total_fee: 0.0,
                total_buys: 0,
                total_sells: 0,
                total_profitable_sells: 0,
                total_stoploss_sells: 0,
            },
            sma9: SimpleMovingAverage::new(9).unwrap(),
            sma26: SimpleMovingAverage::new(26).unwrap(),
            sma50: SimpleMovingAverage::new(50).unwrap(),
            sma200: SimpleMovingAverage::new(200).unwrap(),
            sma201: SimpleMovingAverage::new(201).unwrap(),
            sma1440: SimpleMovingAverage::new(1440).unwrap(),
            sma10080: SimpleMovingAverage::new(10080).unwrap(),
            macd: MovingAverageConvergenceDivergence::new(34, 144, 9).unwrap(),
            obv: OnBalanceVolume::new(),
            rsi: RelativeStrengthIndex::new(14).unwrap(),
            current_buy_price: None,
            latest_buy_timestamp: None,
            latest_sell_timestamp: None,
            latest_close: None,
        }
    }

    pub fn update(&mut self, kline: Kline) -> Result<(), anyhow::Error> {
        let sma9 = self.sma9.next(kline.close);
        let sma26 = self.sma26.next(kline.close);
        let sma50 = self.sma50.next(kline.close);
        let sma200 = self.sma200.next(kline.close);
        let sma201 = self.sma201.next(kline.close);
        let sma1440 = self.sma1440.next(kline.close);
        let sma10080 = self.sma10080.next(kline.close);
        let macd = self.macd.next(kline.close);
        let obv = self.obv.next(&kline);
        let rsi = self.rsi.next(&kline);

        let timestamp = NaiveDateTime::from_timestamp_opt(kline.open_time / 1000, 0)
            .expect("Invalid timestamp");
        let trading_fee = 0.1;

        self.latest_close = Some(kline.close);
        self.stats.updates += 1;

        // buy logic
        if self.current_buy_price.is_none()
            && self.stats.updates >= 201
            && kline.close > sma9
            && sma9 > sma26
            && sma26 > sma50
            && sma50 > sma200
            && sma200 > sma201
        // && sma200 > sma1440
        // && sma1440 > sma10080
        && rsi > 80.0
        // && obv < 0.0
        // && macd.histogram > 0.0
        {
            // info!("Buy {} for {}", self.symbol.name.yellow(), kline.close);
            self.current_buy_price = Some(kline.close);
            self.latest_buy_timestamp = Some(timestamp);
            self.stats.performance -= trading_fee;
            self.stats.total_buys += 1;
            if self.latest_close.is_some_and(|c| c > kline.close) {
                debug!(
                    "BUY with trend: ↓ {}, macd histogram: {}, obv: {}",
                    kline.close, macd.histogram, obv
                );
            } else {
                debug!(
                    "BUY with trend: ↑ {}, macd histogram: {}, obv: {}",
                    kline.close, macd.histogram, obv
                );
            }
            return Ok(());
        }

        // skip sell logic if there is no buy_price
        let Some(current_buy_price) = self.current_buy_price else {
            return Ok(());
        };

        // skip sell logic if there is no buy timestamp
        let Some(latest_buy_timestamp) = self.latest_buy_timestamp else {
            return Ok(());
        };

        let price_change = 100.0 / current_buy_price * kline.close - 100.0;

        // profitable sell logic
        if price_change >= 5.0 {
            let label = "SELL".green();
            let symbol = self.symbol.name.yellow();
            let close = kline.close;
            debug!("{label} {symbol} for {close} (bought at {current_buy_price})");
            self.stats.performance += 5.0;
            self.stats.performance -= trading_fee;
            self.stats.total_fee += trading_fee;
            self.stats.total_sells += 1;
            self.stats.total_profitable_sells += 1;
            self.current_buy_price = None;
            self.latest_sell_timestamp = Some(timestamp);
            return Ok(());
        }

        // stop loss sell because old order
        let age = timestamp - latest_buy_timestamp;
        if age.num_days() >= 60 {
            // TODO: maybe it's not a stop loss sell... CHECK
            let label = "STOP-LOSS SELL (age)".red();
            let symbol = self.symbol.name.yellow();
            let close = kline.close;
            debug!(
                "{label} {symbol} for {close} (bought at {current_buy_price}, {}%)",
                price_change
            );
            self.stats.performance += price_change;
            self.stats.performance -= trading_fee;
            self.stats.total_fee += trading_fee;
            self.stats.total_sells += 1;
            if price_change > 0.0 {
                self.stats.total_profitable_sells += 1;
            } else {
                self.stats.total_stoploss_sells += 1;
            }
            self.current_buy_price = None;
            self.latest_sell_timestamp = Some(timestamp);
            return Ok(());
        }

        Ok(())
    }

    pub fn finalize(&mut self) -> Result<(), anyhow::Error> {
        let Some(latest_close) = self.latest_close else {
            panic!(
                "{} has been finalized but contains no latest_close value",
                self.symbol.name
            );
        };

        // skip if there is no open order
        let Some(current_buy_price) = self.current_buy_price else {
            return Ok(());
        };

        let price_change = 100.0 / current_buy_price * latest_close - 100.0;
        let symbol = self.symbol.name.clone();
        let trading_fee = 0.1;

        // TODO
        // self.latest_sell_timestamp = Some(timestamp);
        self.current_buy_price = None;
        self.stats.performance += price_change;
        self.stats.performance -= trading_fee;
        self.stats.total_fee += trading_fee;
        self.stats.total_sells += 1;

        if price_change > 0.0 {
            let label = "SELL (finalize)".green();
            self.stats.total_profitable_sells += 1;
            debug!(
                "{label} {symbol} for {latest_close} (bought at {current_buy_price}, {}%)",
                price_change
            );
        } else {
            let label = "STOP-LOSS SELL (finalize)".red();
            self.stats.total_stoploss_sells += 1;
            debug!(
                "{label} {symbol} for {latest_close} (bought at {current_buy_price}, {}%)",
                price_change
            );
        }
        Ok(())
    }
}
