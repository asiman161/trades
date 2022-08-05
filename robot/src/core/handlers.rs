use std::time::SystemTime;

use actix_web::{get, http::header::ContentType, HttpResponse, post, Responder, web};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use crate::bns::downloader;
use crate::core::models::{BnsKline, BnsKlinesRequest, BnsPairRequest, NewBnsPair};
use crate::core::storage;

#[get("/api/ping")]
pub async fn ping() -> impl Responder {
    format!("pong")
}

#[post("/api/bns_pair")]
pub async fn create_bns_pair(req: web::Json<BnsPairRequest>) -> impl Responder {
    let s = storage::new();
    let new_pair = s.create_bns_pair(NewBnsPair {
        ticker: if let Some(v) = &req.ticker {v.to_string()} else {String::from("")},
        interval: req.interval.to_string(),
    });

    let body = serde_json::to_string(&new_pair).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[get("/api/bns_pair")]
pub async fn get_bns_pair(req: web::Query<BnsPairRequest>) -> impl Responder {
    let s = storage::new();
    let bns_pair = s.get_bns_pair(if let Some(v) = &req.ticker {v.to_string()} else {String::from("BTCUSDT")}, req.interval.to_string());

    match bns_pair {
        Ok(v) => {
            let body = serde_json::to_string(&v).unwrap();

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body)
        }
        Err(v) => {
            HttpResponse::InternalServerError()
                .body(v.to_string())
        }
    }
}

#[post("api/klines")]
pub async fn get_klines(req: web::Json<BnsKlinesRequest>) -> impl Responder {
    let d = downloader::new("", "");
    let interval = downloader::interval_from_name(req.interval.as_str());
    let now = SystemTime::now();
    let result = d.get_klines(&req.ticker, &interval, req.samples).await;
    println!("spent time: {:?}", now.elapsed());

    // match &result {
    //     Ok(klines) => {
    //         for (i, v) in klines.iter().enumerate() {
    //             let naive = NaiveDateTime::from_timestamp(v.open_time / 1000, (v.open_time % 1000) as u32);
    //             let rfc = chrono::DateTime::<Utc>::from_utc(naive, Utc).to_rfc3339();
    //             println!("size: {}, index: {}: open price: [{}] at [{}]", klines.len(), i, v.open, rfc)
    //         }
    //     }
    //     Err(e) => {}
    // }

    use bigdecimal::Num;
    return match &result {
        Ok(klines) => {
            let store = storage::new();
            let mut db_klines: Vec<BnsKline> = Vec::new();
            for v in klines.iter() {
                let kline = BnsKline {
                    pair: 1,
                    open_time: NaiveDateTime::from_timestamp(v.open_time / 1000, (v.open_time % 1000) as u32),
                    open: BigDecimal::from_str_radix(v.open.as_str(), 10).unwrap(),
                    high: BigDecimal::from_str_radix(v.high.as_str(), 10).unwrap(),
                    low: BigDecimal::from_str_radix(v.low.as_str(), 10).unwrap(),
                    close: BigDecimal::from_str_radix(v.close.as_str(), 10).unwrap(),
                    volume: v.volume.parse().unwrap(),
                    close_time: NaiveDateTime::from_timestamp(v.close_time / 1000, (v.close_time % 1000) as u32),
                    quote_asset_volume: v.quote_asset_volume.parse().unwrap(),
                    number_of_trades: v.number_of_trades as i32,
                    taker_buy_base_asset_volume: v.taker_buy_base_asset_volume.parse().unwrap(),
                    taker_buy_quote_asset_volume: v.taker_buy_quote_asset_volume.parse().unwrap(),
                };
                db_klines.push(kline)
            }

            match store.create_bns_klines(db_klines) {
                Ok(_) => {
                    let body = serde_json::to_string(&result).unwrap();

                    HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .body(body)
                }
                Err(v) => {
                    HttpResponse::InternalServerError().body(v.to_string())
                }
            }
        }
        Err(v) => {
            HttpResponse::InternalServerError().body(v.to_string())
        }
    }
}
