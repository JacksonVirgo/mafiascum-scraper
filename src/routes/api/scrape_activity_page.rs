use crate::components::{
    buttons::{gen_button, ButtonType, FormSubmitButton},
    forms::input::{gen_input, InputType, SelectMenuInput, TextInput},
};
use crate::scraping::{
    parser::{get_search_params, get_url_from_type, PageType, PostURL, ThreadURL, URLType},
    scraper::get_page_details,
};
use actix_web::{post, web, HttpResponse, Responder};
use maud::html;

#[derive(serde::Deserialize)]
pub struct FormData {
    url: String,
}

#[post("/scrape-activity-page")]
async fn scrape_activity_page(form: web::Form<FormData>) -> impl Responder {
    let search_params = get_search_params(&form.url);
    let raw_url = match (search_params.get("t"), search_params.get("p")) {
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

    // Instead of Vec, store the values in particular variables to pre-hydrate the form
    // The values include:
    // - Thread ID
    // - Thread Title
    // - Thread Queue
    // - Total Post Count (for final games)
    //      - Search page on ppp=200&start=9999999999

    let page_data = get_page_details(url.clone()).await;
    let (title, url, thread_id) = if let Some(page) = page_data {
        (
            page.title.as_deref().unwrap_or("").to_owned(),
            page.url.as_deref().unwrap_or("").to_owned(),
            page.thread_id.as_deref().unwrap_or("").to_owned(),
        )
    } else {
        (String::new(), String::new(), String::new())
    };

    HttpResponse::Ok().body(html! ({
        form."text-center w-1/2 flex flex-col items-center justify-left gap-2" hx-post="/api/scrape-activity-page" hx-target="this" hx-indicator="#scrape-form-loading" hx-swap="outerHTML" {
            (gen_input(InputType::SelectMenuInput(SelectMenuInput {
                name: "game_queue".to_string(),
                placeholder: "Select the game queue".to_string(),
                options: vec![String::from("Open"), String::from("Newbie"), String::from("Normal"), String::from("Mini/Micro Theme"), String::from("Large Theme"), String::from("Other/Unknown")],
                is_required: Some(true),
                default_value: Some(String::from("Other/Unknown"))
            })))

            (gen_input(InputType::TextInput(TextInput {
                name: "game_index".to_string(),
                placeholder: "Game Index".to_string(),
                is_required: Some(true),
                default_value: None
            })))

           (gen_input(InputType::TextInput(TextInput {
               name: "title".to_string(),
               placeholder: "Enter the game title".to_string(),
               is_required: Some(true),
               default_value: Some(title)
           })))

            (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                text: "Submit".to_string(),
            })))
        }
    }).into_string())
}
