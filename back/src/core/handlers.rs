use std::time::{SystemTime};

use actix_web::{get, HttpResponse, http::header::ContentType, Responder, web};
use binance::api::Binance;
use binance::market::Market;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::bns::downloader;
use crate::bns::downloader::{Downloader};
use crate::core::models::KlinesRequest;
use crate::models::Post;
use crate::schema::posts::dsl::posts;
use crate::schema::posts::published;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    let connection = crate::establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    let path_param = name.as_str();
    for post in results {
        println!("{}. path param name: {}", post.title, path_param);
        println!("----------\n");
        println!("{}", post.body);
    }
    format!("Hello world!")
}

#[get("/klines")]
pub async fn get_klines(req: web::Query<KlinesRequest>) -> impl Responder {
    let d = Downloader::new("", "");
    let interval = downloader::interval_from_name(req.interval.as_str());
    let now2 = SystemTime::now();
    let result = d.get_klines(&req.ticker, &interval, req.samples).await;
    println!("spent time: {:?}", now2.elapsed());

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

    let body = serde_json::to_string(&result).unwrap();


    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[get("/")]
pub async fn hello() -> impl Responder {
    let m: Market = Binance::new(None, None);
    match m.get_depth("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    format!("Hello world!")
}

#[get("/post")]
pub async fn create_const_post() -> impl Responder {
    let connection = crate::establish_connection();

    println!("What would you like your title to be?");
    let title = String::from("post title");

    let title = &title[..(title.len() - 1)]; // Drop the newline character

    let body = String::from("post title");

    let post = crate::create_post(&connection, title, &body);
    format!("\nSaved draft {} with id {}", title, post.id)
}