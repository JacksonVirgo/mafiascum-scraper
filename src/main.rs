use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use maud::html;

#[get("/")]
async fn hello() -> impl Responder {
    let data = "Test Data";
    let markup = html! {
        p { "Data: " (data) "!" }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}

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
            .service(hello)
    })
    .bind(&address)?
    .run()
    .await
}
