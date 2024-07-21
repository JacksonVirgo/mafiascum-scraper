use reqwest::Client;
use scraper::{Html, Selector};

use crate::scraping::parser::get_search_params;

#[derive(Debug)]
pub struct PageDetails {
    pub title: String,
    pub url: String,
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

    let document = Html::parse_document(&body);

    let user_list_selector = match Selector::parse("#page-body > form > table > tbody") {
        Ok(selector) => selector,
        Err(_) => return None,
    };

    let mut users: Vec<String> = Vec::new();
    let list_element = document.select(&user_list_selector).next()?;

    let row_selector = match Selector::parse("tr") {
        Ok(selector) => selector,
        Err(_) => return None,
    };

    let cell_selector = match Selector::parse("td") {
        Ok(selector) => selector,
        Err(_) => return None,
    };

    let username_selector = match Selector::parse("a > .iso-username") {
        Ok(selector) => selector,
        Err(_) => return None,
    };

    for row in list_element.select(&row_selector) {
        let mut cells = row.select(&cell_selector);
        if let (Some(_first_td), Some(second_td)) = (cells.next(), cells.next()) {
            let mut username_element = second_td.select(&username_selector);
            if let Some(username) = username_element.next() {
                users.push(username.text().collect::<Vec<_>>().concat());
                continue;
            }
        }
    }

    return Some(ActivityPageDetails { users });
}
