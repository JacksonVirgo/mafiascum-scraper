use actix_web::{post, web, HttpResponse, Responder};
use maud::html;

use crate::scraping::{parser::parse_url, scraper::get_page_details};

#[derive(serde::Deserialize)]
pub struct FormData {
    url: String,
}

#[post("/scrape-activity-page")]
async fn scrape_activity_page(form: web::Form<FormData>) -> impl Responder {
    let url = &form.url;

    let mut data: Vec<String> = Vec::new();

    let page_data = get_page_details(url).await;
    match page_data {
        Some(page) => {
            data.push(format!("Title: {}", page.title));
            data.push(format!("URL: {}", page.url));
            if page.thread_id.is_some() {
                data.push(format!(
                    "Thread: {}",
                    page.thread_id.unwrap_or("Error".to_string())
                ));
            } else {
                data.push("Thread: None".to_string());
            }
        }
        None => {}
    };

    if let Some(new_url) = parse_url(url) {
        let markup = match new_url {
            crate::scraping::parser::URLType::Thread(thread) => {
                html! {
                    div {
                        div."text-red-500" { (format!("Thread: {}", thread.thread_id)) }
                        @for string in data {
                            div."text-white" { (string) }
                        }
                    };
                }
            }
            crate::scraping::parser::URLType::Post(post) => {
                html! {
                    div {
                        div."text-red-500" { (format!("Post: {}", post.post_id)) }
                        @for string in data {
                            div."text-white" { (string) }
                        }
                    }
                }
            }
        };

        let html = markup.into_string();
        return HttpResponse::Ok().body(html);
    } else {
        let markup = html! {
            div {
                (format!("Invalid URL: {}", url))
            }
        };
        let html = markup.into_string();
        return HttpResponse::Ok().body(html);
    }
}
