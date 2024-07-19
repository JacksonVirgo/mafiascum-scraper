use crate::components::{
    buttons::{gen_button, ButtonType, ExternalCTAButton},
    header::{generate_header, Header},
};
use actix_web::{get, HttpResponse, Responder};
use maud::html;

#[get("/")]
async fn main() -> impl Responder {
    let header = generate_header(Header {
        title: "MafiaScum Scraper",
    });

    let cta = gen_button(ButtonType::ExternalCTA(ExternalCTAButton {
        text: "Get Started".to_string(),
        link: "/test".to_string(),
    }));

    let markup = html! {
        (header)
        body."bg-zinc-900 w-screen h-screen flex flex-col items-center justify-center" {
            div."text-center w-1/2 flex flex-col items-center justify-center" {
                h1."text-3xl text-white font-bold pb-2" { "MafiaScum Scraper" }
                div."text-xl text-white" {
                    "An easy to use interface for gathering and retrieving game data from mafiascum.net"
                }
                (cta)
            }
        }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}

#[get("/test")]
async fn test() -> impl Responder {
    let header = generate_header(Header {
        title: "MafiaScum Scraper",
    });

    let markup = html! {
        (header)
        body."bg-zinc-900 w-screen h-screen flex flex-col items-center justify-center" {
            div."text-center w-1/2 flex flex-col items-center justify-center" {
                h1."text-3xl text-white font-bold pb-2" { "Test Successful" }
            }
        }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}
