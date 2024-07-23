use reqwest::Client;
use select::document::Document;
use select::predicate::Name;

use crate::scraping::parser::get_search_params;

#[derive(Debug)]
pub struct PageDetails {
    pub title: Option<String>,
    pub url: Option<String>,
    pub thread_id: Option<String>,
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

    let mut page_data = PageDetails {
        title: None,
        url: None,
        thread_id: None,
    };

    match header {
        Some(node) => {
            page_data.title = Some(node.text());
            page_data.url = node
                .find(Name("a"))
                .next()
                .and_then(|node| node.attr("href"))
                .map(String::from);

            if let Some(url) = &page_data.url {
                page_data.thread_id = get_search_params(&url).get("t").map(String::from);
            }
        }
        None => (),
    };

    Some(page_data)
}
