extern crate diesel;
extern crate dotenv;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(index))
        .bind(("127.0.0.1", 8003))?
        .run()
        .await
}

#[derive(Debug, Serialize, Deserialize)]
struct Hello<'a> {
    message: &'a str,
}

// api.
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Hello {
        message: "hello world!",
    })
}

#[get("/books")]
async fn index() -> impl Responder {
    let books = show_books();

    HttpResponse::Ok().json(books)
}

// db access.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn show_books() -> Vec<app::models::Post> {
    use app::schema::posts::dsl::*;

    let connection = establish_connection();
    posts
        .limit(5)
        .load::<app::models::Post>(&connection)
        .expect("Error loading posts")
}
