use reqwest::Client;
use scraper::{Html, Selector};

use crate::scraping::parser::get_search_params;

#[derive(Debug)]
pub struct PageDetails {
    pub title: String,
    pub url: String,
    pub thread_id: Option<String>,
}

pub async fn get_page_details(url: &str) -> Option<PageDetails> {
    let client = Client::new();
    let response = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(_) => return None,
    };

    let body = match response.text().await {
        Ok(text) => text,
        Err(_) => return None,
    };

    let document = Html::parse_document(&body);
    let title_selector = match Selector::parse("h2") {
        Ok(selector) => selector,
        Err(_) => return None,
    };

    let a_selector = match Selector::parse("a") {
        Ok(selector) => selector,
        Err(_) => return None,
    };

    let title_element = document.select(&title_selector).next()?;
    let title = title_element.text().collect::<Vec<_>>().concat();
    let href = title_element
        .select(&a_selector)
        .next()
        .and_then(|a_element| a_element.value().attr("href").map(String::from));

    let thread_id = match href.clone() {
        Some(href) => {
            let search_params = get_search_params(&href);
            match search_params.get("t") {
                Some(thread_id) => Some(thread_id.clone()),
                None => None,
            }
        }
        None => None,
    };

    Some(PageDetails {
        title,
        url: href.unwrap_or(String::from(url)),
        thread_id,
    })
}
