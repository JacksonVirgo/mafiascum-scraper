use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get PORT from env or default to 3000
    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("env.PORT must be an integer");

    let address = format!("127.0.0.1:{}", port);
    println!("Listening on {}", address);

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(&address)?
    .run()
    .await
}
