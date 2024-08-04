use crate::{
    components::header::{generate_header, Header},
    utils::app_state::AppState,
};
use actix_web::{get, web, HttpResponse, Responder};
use maud::html;
use serde::Deserialize;

use crate::models::thread::get_thread;

#[derive(Deserialize)]
struct QueryParams {
    d: Option<String>,
}

#[get("/dashboard")]
async fn dashboard_no_context() -> impl Responder {
    HttpResponse::Found()
        .insert_header(("Location", "/"))
        .finish()
}

#[get("/dashboard/{thread_id}")]
async fn dashboard(
    raw_thread_id: web::Path<String>,
    query: web::Query<QueryParams>,
    state: web::Data<AppState>,
) -> impl Responder {
    let raw_thread_id = raw_thread_id.into_inner();
    let thread_id = match raw_thread_id.parse::<String>() {
        Ok(thread_id) => thread_id.clone(),
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("Location", format!("/")))
                .finish();
        }
    };

    let _ = match get_thread(&state, &thread_id).await {
        Some(thread) => Some(thread),
        None => {
            return HttpResponse::Found()
                .append_header(("Location", format!("/")))
                .finish();
        }
    };

    let header = generate_header(Header {
        title: format!("Dashboard - {}", thread_id).as_str(),
    });

    let gen_url = |url: &str| format!("/api/dashboard/{}/{}", url, thread_id);
    let get_url_param = |param: &str| format!("/dashboard/{}?d={}", thread_id, param);

    let tabs: Vec<(&str, &str)> = vec![("setup", "1"), ("players", "2"), ("votes", "3")];
    let get_htmx_trigger = |trigger: &str| match query.d {
        Some(ref d) => {
            let d_exists = tabs.iter().filter(|(_, tab)| *tab == d).count() > 0;
            let is_default = !d_exists && trigger == "1";
            let is_trigger = d == trigger;
            if is_default || is_trigger {
                "click, load"
            } else {
                "click"
            }
        }
        None => {
            if trigger == "1" {
                "click, load"
            } else {
                "click"
            }
        }
    };

    let markup = html! {
        (header)
        body."bg-zinc-900 w-screen h-screen flex flex-row items-center justify-center text-white" {
            div."bg-zinc-800 border-r border-zinc-600 shrink h-full" {
                ul."w-64 flex flex-col gap-2 p-4"{
                    @for (tab_name, tab_id) in tabs.iter() {
                        li."cursor-pointer" hx-get=(gen_url(tab_name)) hx-target="#dashboard-content" hx-trigger=(get_htmx_trigger(tab_id)) hx-push-url=(get_url_param(tab_id)) {
                            (tab_name.chars().next().map(|c| c.to_uppercase().collect::<String>() + &tab_name[c.len_utf8()..]).unwrap_or(tab_name.to_string()))
                        }
                    }
                }
            }
            div."grow h-full" id="dashboard-content" {}
        }
    };

    let html = markup.into_string();

    HttpResponse::Ok().body(html)
}
