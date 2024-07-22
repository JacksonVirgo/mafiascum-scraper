use reqwest::Client;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

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

#[derive(Debug)]
pub struct ActivityPageDetails {
    pub users: Vec<String>,
}

pub async fn get_activity_page_details(url: String) -> Option<ActivityPageDetails> {
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

    let user_list_selector = Name("tbody");
    let list_node = document.find(user_list_selector).next()?;

    let mut users = Vec::new();

    for row in list_node.find(Name("tr")) {
        let mut cells = row.find(Name("td"));
        if let (Some(_first_td), Some(second_td)) = (cells.next(), cells.next()) {
            if let Some(username_node) = second_td
                .find(Name("a").descendant(Name("span").and(Class("iso-username"))))
                .next()
            {
                users.push(username_node.text());
            }
        }
    }

    Some(ActivityPageDetails { users })
}
