use crate::models::thread::{create_thread, get_thread};
use crate::scraping::{
    parser::{get_search_params, get_url_from_type, PageType, PostURL, ThreadURL, URLType},
    scraper::get_page_details,
};
use crate::AppState;
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use maud::html;

#[derive(serde::Deserialize)]
pub struct FormData {
    url: String,
}

#[post("/search-or-register-thread")]
async fn search_or_register_thread(
    state: Data<AppState>,
    form: web::Form<FormData>,
) -> impl Responder {
    let query_search_params = get_search_params(&form.url);
    let raw_url = match (query_search_params.get("t"), query_search_params.get("p")) {
        (Some(thread_id), _) => get_url_from_type(
            URLType::Thread(ThreadURL {
                thread_id: thread_id.to_string(),
            }),
            PageType::Thread,
        ),
        (None, Some(post_id)) => get_url_from_type(
            URLType::Post(PostURL {
                post_id: post_id.to_string(),
            }),
            PageType::Thread,
        ),
        _ => None,
    };

    let url = match raw_url {
        None => {
            return HttpResponse::BadRequest().body(
                html! {
                    div."text-red-500" { "Invalid URL" }
                }
                .into_string(),
            )
        }
        Some(url) => url,
    };

    let page_data = match get_page_details(url.clone()).await {
        Some(page) => page,
        None => {
            return HttpResponse::BadRequest().body(
                html! {
                    div."text-red-500" { "Invalid URL" }
                }
                .into_string(),
            )
        }
    };

    let thread_id = page_data.thread_id.clone();
    let existing_thread = match get_thread(&state, thread_id.as_str()).await {
        Some(thread) => Some(thread),
        None => {
            let thread = create_thread(&state, thread_id.as_str()).await;
            match thread {
                Some(thread) => Some(thread),
                None => None,
            }
        }
    };

    match existing_thread {
        Some(_) => {
            println!("Found existing thread: {}", thread_id);
            return HttpResponse::Ok()
                .insert_header(("HX-Redirect", format!("/dashboard/{}", thread_id)))
                .finish();
        }
        None => return HttpResponse::NotFound().body("Failed to find or create thread"),
    };
}
