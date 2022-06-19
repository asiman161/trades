use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use back;
use back::models::Post;
use back::schema::posts::dsl::posts;
use back::schema::posts::published;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let connection = back::establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
    format!("Hello world!")
}

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[get("/post")]
async fn create_const_post() -> impl Responder {
    let connection = back::establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::from("post title");

    let title = &title[..(title.len() - 1)]; // Drop the newline character

    let mut body = String::from("post title");


    let post = back::create_post(&connection, title, &body);
    format!("\nSaved draft {} with id {}", title, post.id)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    back::establish_connection();

    println!("db connect established");
    let port = 8080;
    println!("server started on internal port: {port}");
    HttpServer::new(|| {
        App::new()
        .service(greet)
        .service(hello)
        .service(create_const_post)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}