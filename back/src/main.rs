use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("server started on internal port: {port}");
    HttpServer::new(|| {
        App::new()
            .service(back::core::handlers::greet)
            .service(back::core::handlers::hello)
            .service(back::core::handlers::create_const_post)
            .service(back::core::handlers::get_klines)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
