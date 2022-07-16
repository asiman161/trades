use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;
    println!("server started on internal port: {port}");
    HttpServer::new(|| {
        App::new()
            .service(back::core::handlers::get_klines)
            .service(back::core::handlers::get_bns_pair)
            .service(back::core::handlers::create_bns_pair)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
