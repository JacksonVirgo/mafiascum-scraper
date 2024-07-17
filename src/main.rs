use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mime;
mod routes;
mod templates;

const STYLE_CSS: &[u8] = include_bytes!("./static/output.css");
#[get("/style.css")]
async fn serve_css() -> impl Responder {
    HttpResponse::Ok()
        .content_type(mime::TEXT_CSS_UTF_8)
        .body(STYLE_CSS)
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
            .service(serve_css)
            .service(routes::main::main)
            .default_service(web::route().to(routes::not_found::not_found))
    })
    .bind(&address)?
    .run()
    .await
}
