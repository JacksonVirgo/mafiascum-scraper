use actix_files as fs;
use actix_web::{App, HttpServer};

mod routes;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("env.PORT must be an integer");

    let address = format!("127.0.0.1:{}", port);
    println!("Listening on {}", address);

    HttpServer::new(|| {
        App::new()
            .service(routes::main::main)
            .service(fs::Files::new("/static", "./src/static"))
    })
    .bind(&address)?
    .run()
    .await
}
