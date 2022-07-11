use std::ops::Sub;
use std::thread;
use std::time::{Duration, SystemTime};

use binance::api::Binance;
use binance::market::Market;
use binance::model::KlineSummaries::*;
use binance::model::KlineSummary;
use chrono::{Utc};

// #[derive(Debug)]
pub struct Downloader {
    bin_market_client: Market,
}

pub enum Interval {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
}

impl Downloader {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        let v: Market = Binance::new(Some(String::from(api_key)), Some(String::from(secret_key)));
        Downloader { bin_market_client: v }
    }

    pub async fn get_klines(self, symbol: &String, interval: &Interval, samples: u64) -> Result<Vec<KlineSummary>, String> {
        let max_limit = 1000;
        let time_interval = interval_from_seconds(interval) * max_limit;
        let now = chrono::DateTime::<Utc>::from(SystemTime::now().sub(Duration::from_secs(interval_from_seconds(interval) * samples)));

        let subbed = chrono::DateTime::<Utc>::from(SystemTime::now()
            .sub(Duration::from_millis(now.timestamp_millis() as u64)));


        // // TODO: Del, debug
        // println!("sub: {}, ts size: {}, full ts size: {}, tries: {}",
        //          subbed.timestamp(),
        //          interval_from_seconds(interval),
        //          time_interval, subbed.timestamp() as f64 / time_interval as f64);

        let mut left = subbed.timestamp();
        let mut counter: u32 = 0;
        let mut all_klines: Vec<KlineSummary> = Vec::new();
        while left > 0 {
            let from = SystemTime::now().sub(Duration::from_secs(left as u64));
            let from2 = chrono::DateTime::<Utc>::from(from);
            let result = self.bin_market_client.get_klines(symbol.clone(), get_interval_name(&interval), max_limit as u16, from2.timestamp_millis() as u64, None);
            match result.await {
                Ok(AllKlineSummaries(klines)) => {
                    for v in klines.into_iter() {
                        all_klines.push(v);
                    }
                }

                Err(v) => {
                    return Err(String::from(format!("can't get data from binance tickers, {:?}", v)));
                }
            }

            left -= min(left, time_interval as i64);
            counter += 1;
            if counter % 10 == 0 { // to be more polite to the API and don't get banned
                thread::sleep(Duration::from_secs(1))
            }
        }
        Ok(all_klines)
    }
}

const ONE_MINUTE: &str = "1m";
const THREE_MINUTES: &str = "3m";
const FIVE_MINUTES: &str = "5m";
const FIFTEEN_MINUTES: &str = "15m";
const THIRTY_MINUTES: &str = "30m";
const ONE_HOUR: &str = "1h";
const TWO_HOURS: &str = "2h";
const FOUR_HOURS: &str = "4h";
const SIX_HOURS: &str = "6h";
const EIGHT_HOURS: &str = "8h";
const TWELVE_HOURS: &str = "12h";
const ONE_DAY: &str = "1d";

pub fn interval_from_name(name: &str) -> Interval {
    match name {
        ONE_MINUTE => Interval::OneMinute,
        THREE_MINUTES => Interval::ThreeMinutes,
        FIVE_MINUTES => Interval::FiveMinutes,
        FIFTEEN_MINUTES => Interval::FifteenMinutes,
        THIRTY_MINUTES => Interval::ThirtyMinutes,
        ONE_HOUR => Interval::OneHour,
        TWO_HOURS => Interval::TwoHours,
        FOUR_HOURS => Interval::FourHours,
        SIX_HOURS => Interval::SixHours,
        EIGHT_HOURS => Interval::EightHours,
        TWELVE_HOURS => Interval::TwelveHours,
        ONE_DAY => Interval::OneDay,
        _ => Interval::OneMinute
    }
}

fn interval_from_seconds(v: &Interval) -> u64 {
    const MINUTE: u64 = 60;
    const HOUR: u64 = MINUTE * 60;
    const DAY: u64 = HOUR * 24;
    match v {
        Interval::OneMinute => MINUTE,
        Interval::ThreeMinutes => MINUTE * 3,
        Interval::FiveMinutes => MINUTE * 5,
        Interval::FifteenMinutes => MINUTE * 15,
        Interval::ThirtyMinutes => MINUTE * 30,
        Interval::OneHour => HOUR,
        Interval::TwoHours => HOUR * 2,
        Interval::FourHours => HOUR * 4,
        Interval::SixHours => HOUR * 6,
        Interval::EightHours => HOUR * 8,
        Interval::TwelveHours => HOUR * 12,
        Interval::OneDay => DAY,
    }
}

fn get_interval_name(v: &Interval) -> &str {
    match v {
        Interval::OneMinute => ONE_MINUTE,
        Interval::ThreeMinutes => THREE_MINUTES,
        Interval::FiveMinutes => FIVE_MINUTES,
        Interval::FifteenMinutes => FIFTEEN_MINUTES,
        Interval::ThirtyMinutes => THIRTY_MINUTES,
        Interval::OneHour => ONE_HOUR,
        Interval::TwoHours => TWO_HOURS,
        Interval::FourHours => FOUR_HOURS,
        Interval::SixHours => SIX_HOURS,
        Interval::EightHours => EIGHT_HOURS,
        Interval::TwelveHours => TWELVE_HOURS,
        Interval::OneDay => ONE_DAY,
    }
}

fn min<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 < v2 { v1 } else { v2 }
}