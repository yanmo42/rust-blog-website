use actix_web::{web, App, HttpResponse, HttpServer};
use actix_files as fs;
use serde::Serialize;
use tera::{Tera, Context};

#[derive(Serialize, Clone)] // Add `Clone` here
struct BlogPost {
    title: String,
    content: String,
}

async fn index(tera: web::Data<Tera>, posts: web::Data<Vec<BlogPost>>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("title", "My Blog");

    // Convert Arc<Vec<BlogPost>> to Vec<BlogPost> using to_vec()
    let posts_cloned = posts.to_vec();
    context.insert("posts", &posts_cloned);

    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the Tera template engine
    let tera = Tera::new("templates/**/*").unwrap();

    // Initialize the blog posts data
    let posts = web::Data::new(vec![
        BlogPost {
            title: "First Post".to_string(),
            content: "This is the content of the first post.".to_string(),
        },
        BlogPost {
            title: "Second Post".to_string(),
            content: "Here's some content for the second post.".to_string(),
        },
    ]);

    // Configure the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone())) // Share Tera templates
            .app_data(posts.clone()) // Share posts data
            .route("/", web::get().to(index)) // Main route for homepage
            .service(fs::Files::new("/static", "./static").show_files_listing()) // Serve static files
    })
    .bind("127.0.0.1:8080")? // Bind to localhost on port 8080
    .run()
    .await
}
