use actix_web::{post, web, HttpResponse, Responder};
use maud::{html, Markup};

use crate::scraping::{
    parser::{
        get_search_params, get_url_from_type, parse_url, PageType, PostURL, ThreadURL, URLType,
    },
    scraper::get_activity_page_details,
    scraper::get_page_details,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    url: String,
}

#[post("/scrape-activity-page")]
async fn scrape_activity_page(form: web::Form<FormData>) -> impl Responder {
    let search_params = get_search_params(&form.url);
    let url = match (search_params.get("t"), search_params.get("p")) {
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

    match url {
        None => {
            let markup = html! {
                div."text-red-500" { "Invalid URL" }
            };
            let html = markup.into_string();
            return HttpResponse::BadRequest().body(html);
        }
        Some(url) => {
            let mut data: Vec<Markup> = Vec::new();
            let span_closure = |data: String| {
                html! {
                    span { (data) }
                }
            };

            let page_data = get_page_details(url.clone()).await;
            match page_data {
                Some(page) => {
                    data.push(span_closure(format!("Title: {}", page.title)));
                    data.push(span_closure(format!("URL: {}", page.url)));
                    if page.thread_id.is_some() {
                        let thread_id = page.thread_id.unwrap_or("Error".to_string());
                        data.push(span_closure(format!("Thread: {}", thread_id)));

                        let url = get_url_from_type(
                            URLType::Thread(ThreadURL {
                                thread_id: thread_id.clone(),
                            }),
                            PageType::ActivityPage,
                        );

                        let user_list = match url {
                            Some(url) => match get_activity_page_details(url).await {
                                Some(thread) => {
                                    html!({
                                        ul."list-disc pl-5" {
                                            @for user in thread.users {
                                                li{ (user) }
                                            }
                                        }
                                    })
                                }
                                None => {
                                    html!({
                                        div."text-red-500" { "Error fetching users" }
                                    })
                                }
                            },
                            None => {
                                html!({
                                    div."text-red-500" { "Error fetching activity page" }
                                })
                            }
                        };

                        data.push(user_list);
                    } else {
                        data.push(span_closure("Thread: None".to_string()));
                    }
                }
                None => {}
            };

            if let Some(new_url) = parse_url(url.as_str()) {
                let markup = match new_url {
                    crate::scraping::parser::URLType::Thread(thread) => {
                        html! {
                            div {
                                div."text-red-500" { (format!("Thread: {}", thread.thread_id)) }
                                @for string in data {
                                  (string)
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
    }
}
