use std::collections::HashMap;
use url::Url;

pub struct ThreadURL {
    pub thread_id: String,
}

pub struct PostURL {
    pub post_id: String,
}

pub enum URLType {
    Thread(ThreadURL),
    Post(PostURL),
}

pub enum PageType {
    Thread,
}

pub fn get_url_from_type(url_type: URLType, page_type: PageType) -> Option<String> {
    let base_url = "https://forum.mafiascum.net";
    match page_type {
        PageType::Thread => match url_type {
            URLType::Thread(thread) => {
                Some(format!("{}/viewtopic.php?t={}", base_url, thread.thread_id))
            }
            URLType::Post(post) => Some(format!("{}/viewtopic.php?p={}", base_url, post.post_id)),
        },
    }
}

pub fn get_search_params(url: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let base_url = "http://example.com"; // For resolving relative URLs
    let resolved_url = match Url::parse(url) {
        Ok(parsed_url) => parsed_url,
        Err(_) => match Url::parse(base_url) {
            Ok(base) => match base.join(url) {
                Ok(joined_url) => joined_url,
                Err(_) => return params,
            },
            Err(_) => return params,
        },
    };

    for (key, value) in resolved_url.query_pairs() {
        params.insert(key.into_owned(), value.into_owned());
    }

    params
}
