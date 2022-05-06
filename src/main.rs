use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index)
            .service(store)
            .service(update)
            .service(delete)
    })
    .bind(("0.0.0.0", 8083))?
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

#[post("/books")]
async fn store(body: web::Json<app::models::NewPost>) -> impl Responder {
    let book = create_book(body);

    HttpResponse::Ok().json(book)
}

#[put("/books/{book_id}")]
async fn update(
    path_params: web::Path<(i32,)>,
    body: web::Json<app::models::NewPost>,
) -> impl Responder {
    let id = path_params.0;

    let book = update_book(id, body);

    HttpResponse::Ok().json(book)
}

#[delete("/books/{book_id}")]
async fn delete(path_params: web::Path<(i32,)>) -> impl Responder {
    let id = path_params.0;

    delete_book(id);

    HttpResponse::Ok().json({})
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
        .load::<app::models::Post>(&connection)
        .expect("Error loading posts")
}

fn create_book(body: Json<app::models::NewPost>) -> app::models::Post {
    let connection = establish_connection();

    use app::models::NewPost;
    use app::schema::posts;

    let new_post = NewPost {
        title: String::from(&body.title),
        body: String::from(&body.body),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&connection)
        .expect("Error saving new post")
}

fn update_book(id: i32, params: Json<app::models::NewPost>) -> app::models::Post {
    use app::schema::posts::dsl::{body, posts, title};

    let connection = establish_connection();

    diesel::update(posts.find(id))
        .set((
            title.eq(String::from(&params.title)),
            body.eq(String::from(&params.body)),
        ))
        .get_result::<app::models::Post>(&connection)
        .expect(&format!("Unable to find post {}", id))
}

fn delete_book(id: i32) {
    use app::schema::posts::dsl::posts;

    let connection = establish_connection();

    diesel::delete(posts.find(id))
        .get_result::<app::models::Post>(&connection)
        .expect("Error deleting new post");
}
