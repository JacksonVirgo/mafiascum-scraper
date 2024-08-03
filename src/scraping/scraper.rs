use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Or, Predicate};

use crate::scraping::parser::get_search_params;

#[derive(Debug)]
pub struct PageData {
    // pub title: String,
    // pub url: String,
    pub thread_id: String,
    pub votes: Vec<Vote>,
    pub current_page: i32,
    pub last_page: i32,
}

#[derive(Debug)]
pub struct Vote {
    pub author: String,
    pub target: String,
    pub post_number: i32,
}

pub async fn get_page_details(url: String) -> Option<PageData> {
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

    let thread_id = scrape_header(&document);
    let (current_page, last_page) = scrape_pagination(&document);
    let votes = scrape_votes(&document);

    match (thread_id, current_page, last_page) {
        (Some(thread_id), Some(current_page), Some(last_page)) => Some(PageData {
            thread_id,
            votes,
            current_page,
            last_page,
        }),
        _ => None,
    }
}

pub fn scrape_pagination(document: &Document) -> (Option<i32>, Option<i32>) {
    let mut current_page: Option<i32> = None;
    let mut last_page: Option<i32> = None;

    let pagination = match document.find(Class("pagination")).next() {
        Some(node) => node,
        None => return (current_page, last_page),
    };

    let active: i32 = match pagination.find(Class("active")).next() {
        Some(node) => match node.text().parse::<i32>() {
            Ok(page) => page,
            Err(_) => return (current_page, last_page),
        },
        None => return (current_page, last_page),
    };

    let mut largest_page_anchor = 0;
    pagination.find(Name("li")).for_each(|li| {
        match li.find(Name("a")).next() {
            Some(node) => {
                let text = node.text();
                match text.trim().parse::<i32>() {
                    Ok(page) => {
                        if page > largest_page_anchor {
                            largest_page_anchor = page;
                        }
                        ()
                    }
                    Err(_) => (),
                };
            }
            None => return,
        };
    });

    // Get largest value between largest_page_anchor and active
    last_page = Some(largest_page_anchor.max(active));
    current_page = Some(active);
    (current_page, last_page)
}

pub fn scrape_header(document: &Document) -> Option<String> {
    let header = document.find(Name("h2")).next();
    let mut thread_id: Option<String> = None;
    match header {
        Some(node) => {
            let url = node
                .find(Name("a"))
                .next()
                .and_then(|node| node.attr("href"))
                .map(String::from);

            if let Some(url) = url {
                thread_id = get_search_params(&url).get("t").map(String::from);
            }
        }
        None => (),
    };
    thread_id
}

pub fn scrape_votes(document: &Document) -> Vec<Vote> {
    let votes: Vec<Vote> = Vec::new();

    document.find(Class("post")).for_each(|node| {
        let votes: Vec<String> = node
            .find(Or(Class("bbvote"), Name("div").and(Attr("style", ()))))
            .map(|node| node.text())
            .filter(|text| text.to_lowercase().starts_with("vote:"))
            .collect();
        if votes.len() > 0 {
            let author: Option<String> =
                match node.find(Class("username")).collect::<Vec<_>>().first() {
                    Some(node) => Some(node.text()),
                    _ => {
                        match node
                            .find(Class("username-coloured"))
                            .collect::<Vec<_>>()
                            .first()
                        {
                            Some(node) => Some(node.text()),
                            _ => None,
                        }
                    }
                };

            let post_number = match node
                .find(Class("post-number-bolded"))
                .collect::<Vec<_>>()
                .first()
            {
                Some(node) => {
                    let remove_first_char = node.text().chars().skip(1).collect::<String>();
                    match remove_first_char.parse::<i32>() {
                        Ok(num) => Some(num),
                        _ => None,
                    }
                }
                _ => None,
            };

            match (author, post_number) {
                (Some(author), Some(post_number)) => {
                    for vote in votes {
                        println!("{} voted {} in post {}", author, vote, post_number);
                    }
                }
                _ => (),
            }
        }
    });

    votes
}
