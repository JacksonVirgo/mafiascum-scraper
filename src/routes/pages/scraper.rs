use crate::components::{
    buttons::{gen_button, ButtonType, FormSubmitButton},
    forms::input::text_input::TextInputBuilder,
    header::{generate_header, Header},
    spinner::gen_spinner,
};
use actix_web::{get, HttpResponse, Responder};
use maud::html;

#[get("/scraper")]
async fn scraper() -> impl Responder {
    let header = generate_header(Header {
        title: "MafiaScum Scraper",
    });

    let url_input = TextInputBuilder::new()
        .name("url")
        .placeholder("https://mafiascum.net")
        .is_required(true)
        .default_value("https://forum.mafiascum.net/viewtopic.php?t=92678")
        .build_html();

    let markup = html! {
        (header)
        body."bg-zinc-900 w-screen h-screen flex flex-col items-center justify-center text-white" {
            h1 ."text-3xl text-white font-bold pb-2" { "MafiaScum Scraper" }
            div."text-xl text-white pb-2" {
                "Enter a URL to scrape from mafiascum.net"
            }
            form."text-center w-1/2 flex flex-col items-center justify-center" hx-post="/api/search-or-register-thread" hx-target="this" hx-indicator="#scrape-form-loading" hx-swap="outerHTML" {
                (url_input)
                (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                    text: "Submit".to_string(),
                })))
            };

            div."htmx-indicator" id="scrape-form-loading" {
                (gen_spinner())
            }
        }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}
