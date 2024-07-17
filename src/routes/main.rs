use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::templates::header::{generate_header, Header};

#[get("/")]
async fn main() -> impl Responder {
    let header = generate_header(Header { title: "Test" });

    let data = "Test Data";
    let markup = html! {
        (header)
        body."bg-gray-900" {
            p."text-lg text-red-300" { "Data: " (data) "!" }
        }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}
