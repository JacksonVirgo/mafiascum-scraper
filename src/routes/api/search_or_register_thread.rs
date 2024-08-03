use crate::models::thread::{create_thread, get_thread};
use crate::scraping::parser::get_search_params;
use crate::utils::app_state::AppState;
use crate::utils::url::ForumURL;
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

    let invalid_url_response = html! { div."text-red-500" { "Invalid URL" } }.into_string();
    let url = match (query_search_params.get("t"), query_search_params.get("p")) {
        (Some(thread_id), _) => ForumURL::new(thread_id.to_string()),
        (None, Some(post_id)) => match ForumURL::new_from_post(post_id.to_string()).await {
            Some(url) => url,
            None => return HttpResponse::BadRequest().body(invalid_url_response),
        },
        _ => return HttpResponse::BadRequest().body(invalid_url_response),
    };

    let page_data = match url.scrape().await {
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
