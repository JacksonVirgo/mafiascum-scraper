use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mime;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
mod components;
mod models;
mod routes;
mod scraping;

pub struct AppState {
    db: Pool<Postgres>,
}

const STYLE_CSS: &[u8] = include_bytes!("./static/output.css");
#[get("/style.css")]
async fn serve_css() -> impl Responder {
    HttpResponse::Ok()
        .content_type(mime::TEXT_CSS_UTF_8)
        .body(STYLE_CSS)
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
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(serve_css)
            .configure(routes::init)
    })
    .bind(&address)?
    .run()
    .await
}
