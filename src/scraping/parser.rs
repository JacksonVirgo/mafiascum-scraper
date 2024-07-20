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

pub fn parse_url(url_str: &str) -> Option<URLType> {
    if let Ok(parsed_url) = Url::parse(url_str) {
        if let Some((_, id)) = parsed_url.query_pairs().find(|(key, _)| key == "t") {
            return Some(URLType::Thread(ThreadURL {
                thread_id: id.to_string(),
            }));
        }

        if let Some((_, id)) = parsed_url.query_pairs().find(|(key, _)| key == "p") {
            return Some(URLType::Post(PostURL {
                post_id: id.to_string(),
            }));
        }
    }
    None
}
