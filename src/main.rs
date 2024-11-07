use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files as fs;
use serde::Serialize;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to My Blog!")
}


async fn get_posts() -> impl Responder {
    let posts = vec![
        BlogPost {
            title: String::from("My First Post"),
            content: String::from("Hello, this is my first blog post!"),
        },
        BlogPost {
            title: String::from("My Second Post"),
            content: String::from("Here's some more content."),
        },
    ];

    HttpResponse::Ok().json(posts) // response with JSON
}


#[derive(Serialize)]
struct BlogPost {
    title: String,
    content: String,
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // Route for the homepage
            .route("/posts", web::get().to(get_posts))
            .service(fs::Files::new("/static", "./static").show_files_listing()) // Static file handling
    })
    .bind("0.0.0.0:8080")? // Bind to localhost for now
    .run()
    .await
}