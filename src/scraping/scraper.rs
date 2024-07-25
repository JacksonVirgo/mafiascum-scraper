use reqwest::Client;
use select::document::Document;
use select::predicate::Name;

use crate::scraping::parser::get_search_params;

#[derive(Debug)]
pub struct PageDetails {
    pub title: String,
    pub url: String,
    pub thread_id: String,
}

pub async fn get_page_details(url: String) -> Option<PageDetails> {
    let client = Client::new();
    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(_) => return None,
    };

    let body = match response.text().await {
        Ok(text) => text,
        Err(_) => return None,
    };

    let document = Document::from(body.as_str());

    let header = document.find(Name("h2")).next();

    let mut title: Option<String> = None;
    let mut url: Option<String> = None;
    let mut thread_id: Option<String> = None;

    match header {
        Some(node) => {
            title = Some(node.text());
            url = node
                .find(Name("a"))
                .next()
                .and_then(|node| node.attr("href"))
                .map(String::from);

            if let Some(url) = &url {
                thread_id = get_search_params(&url).get("t").map(String::from);
            }
        }
        None => (),
    };

    match (title, url, thread_id) {
        (Some(title), Some(url), Some(thread_id)) => Some(PageDetails {
            title,
            url,
            thread_id,
        }),
        _ => None,
    }
}
