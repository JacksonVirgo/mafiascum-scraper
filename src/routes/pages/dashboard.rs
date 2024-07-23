use crate::components::header::{generate_header, Header};
use actix_web::{get, HttpResponse, Responder};
use maud::html;

#[get("/dashboard")]
async fn dashboard() -> impl Responder {
    let header = generate_header(Header {
        title: "Dashboard | MafiaScum Scraper",
    });

    let markup = html! {
        (header)
        body."bg-zinc-900 w-screen h-screen flex flex-row items-center justify-center text-white" {
            div."bg-zinc-800 border-r border-zinc-600 shrink h-full" {
                ul."w-64 flex flex-col gap-2 p-4"{
                    li."cursor-pointer" hx-get="/api/dashboard/setup" hx-target="#dashboard-content" hx-trigger="click, load" {
                        "Setup"
                    }
                    li."cursor-pointer" hx-get="/api/dashboard/players" hx-target="#dashboard-content" {
                        "Players"
                    }
                    li."cursor-pointer" hx-get="/api/dashboard/votes" hx-target="#dashboard-content" {
                        "Votes"
                    }
                }
            }
            div."grow h-full" id="dashboard-content" {}
        }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}
