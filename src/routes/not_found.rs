use actix_web::HttpResponse;
use maud::html;

use crate::components::header::{generate_header, Header};

pub async fn not_found() -> HttpResponse {
    let header = generate_header(Header { title: "Test" });
    let markup = html! {
        (header)
        p { "Error 404 - Page Not Found" }
    };
    let html = markup.into_string();
    HttpResponse::Ok().body(html)
}
