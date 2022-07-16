use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::{bns_klines, bns_pairs};

#[derive(Debug, Serialize, Deserialize)]
pub struct BnsKlinesRequest {
    pub ticker: String,
    pub interval: String,
    pub samples: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BnsPairRequest {
    pub ticker: String,
    pub interval: String,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "bns_klines"]
pub struct BnsKline {
    pub pair: i32,
    pub open_time: NaiveDateTime,
    pub open: BigDecimal,
    pub high: BigDecimal,
    pub low: BigDecimal,
    pub close: BigDecimal,
    pub volume: f32,
    pub close_time: NaiveDateTime,
    pub quote_asset_volume: f32,
    pub number_of_trades: i32,
    pub taker_buy_base_asset_volume: f32,
    pub taker_buy_quote_asset_volume: f32,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct BnsPair {
    pub id: i32,
    pub ticker: String,
    pub interval: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="bns_pairs"]
pub struct NewBnsPair {
    pub ticker: String,
    pub interval: String,
}