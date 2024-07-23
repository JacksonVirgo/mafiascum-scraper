use crate::components::header::{generate_header, Header};
use actix_web::{get, web, HttpResponse, Responder};
use maud::html;

#[get("/dashboard")]
async fn dashboard_no_context() -> impl Responder {
    HttpResponse::Found()
        .insert_header(("Location", "https://mafiascum.net/viewtopic.php?t=92678"))
        .finish()
}

#[get("/dashboard/{thread_id}")]
async fn dashboard(thread_id: web::Path<i32>) -> impl Responder {
    let header = generate_header(Header {
        title: format!("Dashboard - {}", thread_id).as_str(),
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
