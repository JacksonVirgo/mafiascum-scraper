use actix_web::{post, web, HttpResponse, Responder};
use maud::html;

use crate::scraping::parser::parse_url;

#[derive(serde::Deserialize)]
pub struct FormData {
    url: String,
}

#[post("/scrape-activity-page")]
async fn scrape_activity_page(form: web::Form<FormData>) -> impl Responder {
    let url = &form.url;
    if let Some(new_url) = parse_url(url) {
        let markup = match new_url {
            crate::scraping::parser::URLType::Thread(thread) => {
                html! {
                    div {
                        "Thread: " (thread.thread_id)
                    }
                }
            }
            crate::scraping::parser::URLType::Post(post) => {
                html! {
                    div {
                        "Post: " (post.post_id)
                    }
                }
            }
        };

        let html = markup.into_string();
        return HttpResponse::Ok().body(html);
    } else {
        let markup = html! {
            div {
                "Invalid URL: " (url)
            }
        };
        let html = markup.into_string();
        return HttpResponse::Ok().body(html);
    }
}
