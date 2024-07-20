use crate::components::{
    buttons::{gen_button, ButtonType, FormSubmitButton},
    forms::input::{gen_input, InputType, TextInput},
    header::{generate_header, Header},
};
use actix_web::{get, HttpResponse, Responder};
use maud::html;

#[get("/scraper")]
async fn scraper() -> impl Responder {
    let header = generate_header(Header {
        title: "MafiaScum Scraper",
    });

    let markup = html! {
        (header)
        body."bg-zinc-900 w-screen h-screen flex flex-col items-center justify-center" {
            h1 ."text-3xl text-white font-bold pb-2" { "MafiaScum Scraper" }
            div."text-xl text-white pb-2" {
                "Enter a URL to scrape from mafiascum.net"
            }
            form."text-center w-1/2 flex flex-col items-center justify-center" hx-post="/api/scrape-activity-page" hx-target="#response" {
                (gen_input(InputType::TextInput(TextInput {
                    name: "url".to_string(),
                    placeholder: "https://mafiascum.net".to_string(),
                    is_required: Some(true),
                })))
                (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                    text: "Submit".to_string(),
                })))
            };

            div."text-white" id="response" {
                "Response Here"
            }
        }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}
