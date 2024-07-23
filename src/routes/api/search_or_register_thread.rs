use crate::scraping::{
    parser::{get_search_params, get_url_from_type, PageType, PostURL, ThreadURL, URLType},
    scraper::get_page_details,
};
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use maud::html;

use crate::models::thread::get_thread;

use crate::AppState;

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

    let thread_id = match get_page_details(url.clone()).await {
        Some(page) => page.thread_id,
        None => None,
    };

    if let Some(t) = thread_id {
        let thread: Option<crate::models::thread::Thread> = get_thread(state, t.clone()).await;
        println!("Found thread: {:?}", thread);
    } else {
        println!("No thread was found");
    }

    HttpResponse::Ok().body(html! {}.into_string())
}
