use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mafiascum_scraper::{routes, utils::app_state::AppState, utils::logger};
use mime;
use sqlx::postgres::PgPoolOptions;

// TODO: Remove hardcoded static files, add to custom route
const STYLE_CSS: &[u8] = include_bytes!("./static/output.css");
const FAVICON: &[u8] = include_bytes!("./static/favicon.ico");
#[get("/style.css")]
async fn serve_css() -> impl Responder {
    HttpResponse::Ok()
        .content_type(mime::TEXT_CSS_UTF_8)
        .body(STYLE_CSS)
}

#[get("/favicon.ico")]
async fn serve_favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(FAVICON)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error building a connection pool.");

    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("env.PORT must be an integer");

    let address = format!("0.0.0.0:{}", port);
    println!("Listening on {}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(logger::Logger)
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(serve_css)
            .service(serve_favicon)
            .configure(routes::init)
    })
    .bind(&address)?
    .run()
    .await
}
