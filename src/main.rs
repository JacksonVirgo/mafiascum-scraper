use actix_files as fs;
use actix_web::{web, App, HttpServer};

mod routes;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("env.PORT must be an integer");

    let address = format!("0.0.0.0:{}", port);
    println!("Listening on {}", address);

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./src/static").show_files_listing())
            .service(routes::main::main)
        // .default_service(web::route().to(routes::not_found::not_found))
    })
    .bind(&address)?
    .run()
    .await
}
